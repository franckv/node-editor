use std::fmt::{Debug, Display};

use crate::node::NodeValue;

pub struct Template;

impl Template {
    pub fn builder(pattern: &str) -> TemplateBuilder {
        TemplateBuilder {
            pattern: pattern.to_string(),
        }
    }
}

pub struct TemplateBuilder {
    pattern: String,
}

impl TemplateBuilder {
    pub fn float(mut self, param: &str, value: f32) -> Self {
        let value = NodeValue::format_float(value);
        let param = format!("{{{}}}", param);

        self.pattern = self.pattern.replace(&param, &value);

        self
    }

    pub fn param<D: Display>(mut self, param: &str, value: D) -> Self {
        let value = format!("{}", value).to_lowercase();
        let param = format!("{{{}}}", param);

        self.pattern = self.pattern.replace(&param, &value);

        self
    }

    pub fn param_dbg<D: Debug>(mut self, param: &str, value: D) -> Self {
        let value = format!("{:?}", value).to_lowercase();
        let param = format!("{{{}}}", param);

        self.pattern = self.pattern.replace(&param, &value);

        self
    }

    pub fn build(self) -> String {
        self.pattern
    }
}
