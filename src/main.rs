use keyring::Entry;
use ldap3::{LdapConn, Scope, SearchEntry};
use nu_plugin::{serve_plugin, EvaluatedCall, LabeledError, MsgPackSerializer, Plugin};
use nu_protocol::{record, Category, PluginSignature, Spanned, SyntaxShape, Value};
use std::vec;
struct MyPlugin;

impl Plugin for MyPlugin {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![PluginSignature::build("ldap")
            .usage("Query user form PLN ldap server.")
            .named("url", SyntaxShape::String, "ldap server url", Some('u'))
            .required("query", SyntaxShape::String, "ldap search query")
            .category(Category::Misc)]
    }
    fn run(
        &mut self,
        _name: &str,
        _config: &Option<Value>,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        // print!("{:#?}", call);
        let params: Spanned<String> = call.req(0)?;
        let url = call
            .get_flag::<String>("url")?
            .unwrap_or("ldap://10.1.8.20:389".into());
        // dbg!(params.item);
        let mut ldap = LdapConn::new(url.as_str()).unwrap();
        // Entry::new_with_target("pln", service, user -> whoami)
        let entry = match Entry::new_with_target("pln", "keyring-cli", whoami::username().as_str())
        {
            Ok(entry) => entry,
            Err(_) => {
                return Err(LabeledError {
                    label: "Error".into(),
                    msg: "Failed to get keyring entry".into(),
                    span: Some(call.head),
                })
            }
        };
        let password = match entry.get_password() {
            Ok(pass) => pass,
            Err(_) => {
                return Err(LabeledError {
                    label: "Error".into(),
                    msg: "Failed to get password from keyring storage".into(),
                    span: Some(call.head),
                })
            }
        };
        let _ = ldap.simple_bind("pusat\\darwis2", password.as_str());
        let res = ldap.search(
            "DC=pusat,DC=corp,DC=pln,DC=co,DC=id",
            Scope::Subtree,
            &params.item,
            // vec!["*"],
            vec![
                "l",
                "title",
                "personalTitle",
                "sAMAccountName",
                "name",
                "displayName",
                "givenName",
                "description",
                "employeeType",
                "mail",
                "memberOf",
                "st",
                "company",
                "streetAddress",
                "department",
                "mobile",
                "sAMAccountName",
                "employeeNumber",
                "dn",
                "lastLogon",
                "distinguishedName",
            ],
        );

        match res {
            Ok(result) => {
                let (rs, _res) = result.success().unwrap();
                // let cols = vec![
                //     "name".to_string(),
                //     // "employeeNumber".to_string(),
                //     "description".to_string(),
                //     "department".to_string(),
                //     "title".to_string(),
                //     "company".to_string(),
                //     "mobile".to_string(),
                //     "employeeType".to_string(),
                // ];
                
                
                // let vals = cols.iter().map(|c| Value::String { val: SearchEntry::construct(p.clone()).attrs.get(c).unwrap_or(&vec![String::new()]).first().unwrap().to_string(), internal_span: call.head }).collect();
                // let list_val = rs.iter().map(|p| Value::record(Record { cols: cols.clone(), vals: vals }, call.head)).collect();

                let list_val = rs.iter().map(|p| Value::record(record! {
                    "name" => Value::String { val: SearchEntry::construct(p.clone()).attrs.get("name").unwrap_or(&vec![String::new()]).first().unwrap().to_string(), internal_span: call.head },
                    "description" => Value::String { val: SearchEntry::construct(p.clone()).attrs.get("description").unwrap_or(&vec![String::new()]).first().unwrap().to_string(), internal_span: call.head },
                    "department" => Value::String { val: SearchEntry::construct(p.clone()).attrs.get("department").unwrap_or(&vec![String::new()]).first().unwrap().to_string(), internal_span: call.head },
                    "title" => Value::String { val: SearchEntry::construct(p.clone()).attrs.get("title").unwrap_or(&vec![String::new()]).first().unwrap().to_string(), internal_span: call.head },
                    "company" => Value::String { val: SearchEntry::construct(p.clone()).attrs.get("company").unwrap_or(&vec![String::new()]).first().unwrap().to_string(), internal_span: call.head },
                    "mobile" => Value::String { val: SearchEntry::construct(p.clone()).attrs.get("mobile").unwrap_or(&vec![String::new()]).first().unwrap().to_string(), internal_span: call.head },
                    "employeeType" => Value::String { val: SearchEntry::construct(p.clone()).attrs.get("employeeType").unwrap_or(&vec![String::new()]).first().unwrap().to_string(), internal_span: call.head },
                }, call.head)).collect();

                Ok(Value::List { vals: list_val, internal_span: call.head })


                },
            Err(_err) => Err(LabeledError {
                label: "Plugin call with wrong name signature".into(),
                msg: "the signature used to call the plugin does not match any name in the plugin signature vector".into(),
                span: Some(call.head),
            })
        }
    }
}

fn main() {
    serve_plugin(&mut MyPlugin {}, MsgPackSerializer)
}
