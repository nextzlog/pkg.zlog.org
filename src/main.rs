/*******************************************************************************
 * Amateur Radio Operational Logging Software 'ZyLO' since 2020 June 22nd
 * Released under the MIT License (or GPL v3 until 2021 Oct 28th) (see LICENSE)
 * Univ. Tokyo Amateur Radio Club Development Task Force (https://nextzlog.dev)
*******************************************************************************/

use jsonschema::Validator;
use reqwest::blocking::get;
use serde_json::Serializer;
use serde_yaml::from_str;
use toml::Deserializer;
use toml::Value;

type Return<E> = Result<E, Box<dyn std::error::Error>>;

const SCH: &str = include_str!("schema.yaml");

fn checksum(item: &mut Value) -> Return<()> {
	if item.get("sum").is_none() {
		let val = item.as_table_mut().unwrap();
		let url = val["url"].as_str().unwrap();
		let bin = get(url)?.error_for_status()?.bytes()?;
		let sum = format!("{:x}", md5::compute(bin));
		val.insert("sum".into(), Value::String(sum));
	}
	Ok(())
}

fn document(item: &mut Value) -> Return<()> {
	if item.get("doc").is_some() {
		let val = item.as_table_mut().unwrap();
		let url = val["doc"].as_str().unwrap();
		let txt = get(url)?.error_for_status()?.text()?;
		val.insert("doc".into(), Value::String(txt));
	}
	Ok(())
}

fn tree(list: &mut Value) -> Return<String> {
	let items = list.as_table_mut();
	for (_, it) in items.unwrap() {
		if it.is_table() {
			tree(it)?;
		}
		if it.get("url").is_some() {
			checksum(it)?;
		} else {
			document(it)?;
		}
	}
	Ok(list.to_string())
}

fn fetch(url: &str) -> Return<String> {
	let res = get(url)?.error_for_status()?;
	let val = res.text()?.parse::<Value>()?;
	let cmp = Validator::new(&from_str(SCH)?);
	let tmp = serde_json::to_value(val.clone())?;
	if let Err(error) = cmp?.validate(&tmp) {
		eprintln!("{}", error);
		std::process::abort();
	}
	tree(&mut val.clone())
}

fn merge() -> Return<String> {
	let mut toml = String::new();
	for url in include_str!("market.list").lines() {
		toml.push_str(&format!("{}\n", fetch(url)?));
	}
	Ok(toml)
}

fn main() -> Return<()> {
	let source = merge()?;
	let target = std::io::stdout();
	let mut de = Deserializer::new(&source);
	let mut en = Serializer::pretty(target);
	Ok(serde_transcode::transcode(&mut de, &mut en)?)
}
