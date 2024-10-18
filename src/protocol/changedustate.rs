use std::io::Write;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

use super::{cwmp_prefix, write_simple, GenerateError, InstallOp, UninstallOp, UpdateOp};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct ChangeDUState {
    pub command_key: String,
    pub install_operations: Vec<InstallOp>,
    pub uninstall_operations: Vec<UninstallOp>,
    pub update_operations: Vec<UpdateOp>,
}

impl ChangeDUState {
    #[must_use] pub fn new(
        command_key: String,
        install_operations: Vec<InstallOp>,
        uninstall_operations: Vec<UninstallOp>,
        update_operations: Vec<UpdateOp>,
    ) -> Self {
        ChangeDUState {
            command_key,
            install_operations,
            uninstall_operations,
            update_operations,
        }
    }

    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "ChangeDUState")[..],
        ))?;
        write_simple(writer, "CommandKey", &self.command_key)?;
        writer.write(XmlEvent::start_element("Operations"))?;

        for io in &self.install_operations {
            writer.write(XmlEvent::start_element("InstallOpStruct"))?;
            write_simple(writer, "URL", &io.url)?;
            write_simple(writer, "UUID", &io.uuid)?;
            write_simple(writer, "Username", &io.username)?;
            write_simple(writer, "Password", &io.password)?;
            write_simple(writer, "ExecutionEnvRef", &io.execution_env_ref)?;
            writer.write(XmlEvent::end_element())?;
        }
        for uio in &self.uninstall_operations {
            writer.write(XmlEvent::start_element("UninstallOpStruct"))?;
            write_simple(writer, "URL", &uio.url)?;
            write_simple(writer, "UUID", &uio.uuid)?;
            write_simple(writer, "ExecutionEnvRef", &uio.execution_env_ref)?;
            writer.write(XmlEvent::end_element())?;
        }
        for uo in &self.update_operations {
            writer.write(XmlEvent::start_element("UpdateOpStruct"))?;
            write_simple(writer, "URL", &uo.url)?;
            write_simple(writer, "UUID", &uo.uuid)?;
            write_simple(writer, "Username", &uo.username)?;
            write_simple(writer, "Password", &uo.password)?;
            write_simple(writer, "Version", &uo.version)?;
            writer.write(XmlEvent::end_element())?;
        }

        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::end_element())?;

        Ok(())
    }

    pub fn start_handler(
        &mut self,
        path: &[&str],
        _name: &xml::name::OwnedName,
        _attributes: &Vec<xml::attribute::OwnedAttribute>,
    ) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        match &path_pattern[..] {
            ["ChangeDUState", "Operations", "InstallOpStruct"] => {
                self.install_operations.push(InstallOp::new(
                    String::new(),
                    String::new(),
                    String::new(),
                    String::new(),
                    String::new(),
                ));
            }
            ["ChangeDUState", "Operations", "UninstallOpStruct"] => self.uninstall_operations.push(
                UninstallOp::new(String::new(), String::new(), String::new()),
            ),
            ["ChangeDUState", "Operations", "UpdateOpStruct"] => {
                self.update_operations.push(UpdateOp::new(
                    String::new(),
                    String::new(),
                    String::new(),
                    String::new(),
                    String::new(),
                ));
            }
            _ => {}
        }
    }

    pub fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["ChangeDUState", "CommandKey"] => self.command_key = characters.to_string(),
            ["ChangeDUState", "Operations", "InstallOpStruct", key] => {
                let last = self.install_operations.last_mut();
                if let Some(e) = last { match key {
                    "URL" => e.url = characters.to_string(),
                    "UUID" => e.uuid = characters.to_string(),
                    "Username" => e.username = characters.to_string(),
                    "Password" => e.password = characters.to_string(),
                    "ExecutionEnvRef" => e.execution_env_ref = characters.to_string(),
                    _ => {}
                } }
            }
            ["ChangeDUState", "Operations", "UninstallOpStruct", key] => {
                let last = self.uninstall_operations.last_mut();
                if let Some(e) = last { match key {
                    "URL" => e.url = characters.to_string(),
                    "UUID" => e.uuid = characters.to_string(),
                    "ExecutionEnvRef" => e.execution_env_ref = characters.to_string(),
                    _ => {}
                } }
            }
            ["ChangeDUState", "Operations", "UpdateOpStruct", key] => {
                let last = self.update_operations.last_mut();
                if let Some(e) = last { match key {
                    "URL" => e.url = characters.to_string(),
                    "UUID" => e.uuid = characters.to_string(),
                    "Username" => e.username = characters.to_string(),
                    "Password" => e.password = characters.to_string(),
                    "Version" => e.version = characters.to_string(),
                    _ => {}
                } }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for ChangeDUState {
    fn arbitrary(g: &mut Gen) -> Self {
        ChangeDUState::new(
            String::arbitrary(g),
            Vec::<InstallOp>::arbitrary(g),
            Vec::<UninstallOp>::arbitrary(g),
            Vec::<UpdateOp>::arbitrary(g),
        )
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (
                self.command_key.clone(),
                self.install_operations.clone(),
                self.uninstall_operations.clone(),
                self.update_operations.clone(),
            )
                .shrink()
                .map(|(c, i, un, up)| ChangeDUState {
                    command_key: c,
                    install_operations: i,
                    uninstall_operations: un,
                    update_operations: up,
                }),
        )
    }
}
