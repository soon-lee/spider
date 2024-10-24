pub enum ParamValue {
    Domain(Vec<String>),
    Range((i32, i32)),
}
impl ParamValue {
    pub fn new_domain(domains: Vec<String>) -> Self {
        Self::Domain(domains)
    }
    pub fn new_range(begin: i32, end:i32) -> Self {
        Self::Range((begin,end))
    }
}
pub struct Param {
    name: String,
    value: ParamValue,
}
impl Param {
    pub fn new(name: String, value: ParamValue) -> Self {
        Param { name, value }
    }
    pub fn replace(&self,pattern :&str) -> Vec<String> {
        let mut result = vec![];
        match &self.value {
            ParamValue::Domain(domains) => {
                for domain in domains {
                    result.push(
                        pattern.replace(
                            format!("{{{}}}", self.name).as_str(),
                            domain,
                        ),
                    );
                }
            }
            ParamValue::Range((start, end)) => {
                for i in *start..=*end {
                    result.push(
                        pattern.replace(
                            format!("{{{}}}", self.name).as_str(),
                            i.to_string().as_str(),
                        ),
                    );
                }
            }
        }
        result
    }
}
pub struct ParadedUrl {
    url: String,
    params: Vec<Param>,
}
impl ParadedUrl {
    pub fn new(pattern: String, params: Vec<Param>) -> Self {
        ParadedUrl { url: pattern, params }
    }
}
pub enum PathList {
    Pure(Vec<String>),
    Pattern(ParadedUrl),
}
impl PathList {
    pub fn new_pure(paths: Vec<String>) -> Self {
        Self::Pure(paths)
    }
    pub fn new_pattern(patterned: ParadedUrl) -> Self {
        Self::Pattern(patterned)
    }
    pub fn list(&self) -> Vec<String> {
        match self {
            PathList::Pure(paths) => paths.clone(),
            PathList::Pattern(paraded) => {
                let mut result = vec![];
                for param in &paraded.params {
                    if result.is_empty() {
                        for item in param.replace(&paraded.url){
                            result.push(item);
                        }
                    } else {
                        let mut temp = vec![];
                        loop{
                            let item = result.remove(0);
                            for item in param.replace(&item){
                                temp.push(item);
                            }
                            if result.is_empty(){
                                result = temp;
                                break;
                            }
                        }
                    }
                }
                result
            }
        }
    }
}