use util::escape_text;

#[derive(Clone)]
pub struct Fragment {
    pub data: String,
    element: bool
}

impl<'a> From<&'a str> for Fragment {
    fn from(item: &'a str) -> Self {
        Fragment { data: escape_text(item), element: true }
    }
}

impl From<String> for Fragment {
    fn from(item: String) -> Self {
        Fragment { data: escape_text(&item), element: true }
    }
}

impl From<i32> for Fragment {
    fn from(item: i32) -> Self {
        Fragment { data: item.to_string(), element: true }
    }
}

pub fn raw(value: &str) -> Fragment {
    Fragment { data: value.to_string(), element: true }
}

fn escape_tag_name(name: &str) -> String {
    // @TODO
    name.to_string()
}

fn escape_attribute_name(value: &str) -> String {
    // @TODO
    value.to_string()
}

fn escape_attribute_value(value: &str) -> String {
    // @TODO
    value.to_string()
}


pub fn attribute(name: &str, value: &str) -> Fragment {
    Fragment {
        data: format!("{0}=\"{1}\"", escape_attribute_name(name), escape_attribute_value(value)),
        element: false
    }
}

pub fn empty_attribute(name: &str) -> Fragment {
    Fragment {
        data: format!("{0}", escape_attribute_name(name)),
        element: false
    }
}

pub fn tag(name: &str, unary: bool, children: Vec<Fragment>) -> Fragment {
    // @TODO way to partition here?

    let elements: Vec<String> =
        children
          .iter()
          .filter(|c| c.element)
          .map(|c| c.data.clone())
          .collect();

    let attributes: Vec<String> =
        children
          .iter()
          .filter(|c| !c.element)
          .map(|c| c.data.clone())
          .collect();

    let elements_data = elements.join("");

    let attributes_data =
        if attributes.is_empty() {
            format!("")
        } else {
            format!(" {}", attributes.join(" "))
        };

    let data =
        if unary {
            format!("<{0}{1}/>", escape_tag_name(name), attributes_data)
        } else {
            format!("<{0}{1}>{2}</{0}>", escape_tag_name(name), attributes_data, elements_data)
        };

    Fragment { data: data, element: true }
}
