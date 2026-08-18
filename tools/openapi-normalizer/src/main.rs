use std::{env, fs, path::PathBuf};

use anyhow::{Context, Result, bail};
use serde_json::Value;

fn main() -> Result<()> {
    let mut args = env::args_os().skip(1);
    let Some(input) = args.next() else {
        bail!("usage: openapi-normalizer INPUT OUTPUT");
    };
    let Some(output) = args.next() else {
        bail!("usage: openapi-normalizer INPUT OUTPUT");
    };
    if args.next().is_some() {
        bail!("usage: openapi-normalizer INPUT OUTPUT");
    }

    let input = PathBuf::from(input);
    let output = PathBuf::from(output);
    let source = fs::read_to_string(&input)
        .with_context(|| format!("failed to read {}", input.display()))?;
    let mut spec: Value = serde_yaml::from_str(&source)
        .with_context(|| format!("failed to parse {}", input.display()))?;

    let paths = spec
        .get_mut("paths")
        .and_then(Value::as_object_mut)
        .context("OpenAPI document has no paths object")?;

    repair_known_paths(paths);

    for path_item in paths.values_mut().filter_map(Value::as_object_mut) {
        for operation in path_item.values_mut().filter_map(Value::as_object_mut) {
            if let Some(responses) = operation
                .get_mut("responses")
                .and_then(Value::as_object_mut)
            {
                responses.remove("default");
            }
        }
    }

    let normalized = serde_json::to_string_pretty(&spec)?;
    fs::write(&output, normalized)
        .with_context(|| format!("failed to write {}", output.display()))?;

    Ok(())
}

fn repair_known_paths(paths: &mut serde_json::Map<String, Value>) {
    // protoc-gen-openapi emits the instance-setting wildcard binding as
    // `/instance/{instance}/*`; the v0.30 REST gateway exposes the concrete
    // `/instance/settings/{instance}` route instead.
    if let Some(path_item) = paths.remove("/api/v1/instance/{instance}/*") {
        paths.insert("/api/v1/instance/settings/{instance}".into(), path_item);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repairs_v030_instance_setting_wildcard_path() {
        let mut paths = serde_json::Map::new();
        paths.insert(
            "/api/v1/instance/{instance}/*".into(),
            Value::Object(serde_json::Map::new()),
        );

        repair_known_paths(&mut paths);

        assert!(!paths.contains_key("/api/v1/instance/{instance}/*"));
        assert!(paths.contains_key("/api/v1/instance/settings/{instance}"));
    }
}
