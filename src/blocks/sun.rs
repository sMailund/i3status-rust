use crate::{blocks::prelude::Value, widget::Widget};

use super::prelude::*;

#[derive(Deserialize, Debug, SmartDefault)]
#[serde(deny_unknown_fields, default)]
pub struct Config {
    pub format: FormatConfig,
    #[default(5.into())]
    pub interval: Seconds,
}

pub async fn run(config: &Config, api: &CommonApi) -> Result<()> {
    let format = config.format.with_default("test $testvalue")?;
    loop {
        let mut widget = Widget::new().with_format(format.clone());

        let value = 69;

        widget.set_values(map! {
            "testvalue" => Value::bytes(value as f64),
        });

        api.set_widget(widget)?;
        select! {
            _ = sleep(config.interval.0) => (),
            _ = api.wait_for_update_request() => (),
        }
    }
}
