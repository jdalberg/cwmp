use std::io::Write;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

use super::{
    cwmp_prefix, write_simple, GenerateError, InstallOp, UninstallOp, UpdateOp, XmlSafeString,
};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct ChangeDUState {
    pub command_key: XmlSafeString,
    pub install_operations: Vec<InstallOp>,
    pub uninstall_operations: Vec<UninstallOp>,
    pub update_operations: Vec<UpdateOp>,
}

impl ChangeDUState {
    #[must_use]
    pub fn new(
        command_key: &str,
        install_operations: Vec<InstallOp>,
        uninstall_operations: Vec<UninstallOp>,
        update_operations: Vec<UpdateOp>,
    ) -> Self {
        ChangeDUState {
            command_key: command_key.into(),
            install_operations,
            uninstall_operations,
            update_operations,
        }
    }

    /// Generate XML for `ChangeDUState`
    ///     
    /// # Errors
    ///     Any errors encountered while writing to `writer` will be returned.
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "ChangeDUState")[..],
        ))?;
        write_simple(writer, "CommandKey", self.command_key.0.as_ref())?;
        writer.write(XmlEvent::start_element("Operations"))?;

        for io in &self.install_operations {
            writer.write(XmlEvent::start_element("InstallOpStruct"))?;
            write_simple(writer, "URL", io.url.0.as_ref())?;
            write_simple(writer, "UUID", io.uuid.0.as_ref())?;
            write_simple(writer, "Username", io.username.0.as_ref())?;
            write_simple(writer, "Password", io.password.0.as_ref())?;
            write_simple(writer, "ExecutionEnvRef", io.execution_env_ref.0.as_ref())?;
            writer.write(XmlEvent::end_element())?;
        }
        for uio in &self.uninstall_operations {
            writer.write(XmlEvent::start_element("UninstallOpStruct"))?;
            write_simple(writer, "URL", uio.url.0.as_ref())?;
            write_simple(writer, "UUID", uio.uuid.0.as_ref())?;
            write_simple(writer, "ExecutionEnvRef", uio.execution_env_ref.0.as_ref())?;
            writer.write(XmlEvent::end_element())?;
        }
        for uo in &self.update_operations {
            writer.write(XmlEvent::start_element("UpdateOpStruct"))?;
            write_simple(writer, "URL", uo.url.0.as_ref())?;
            write_simple(writer, "UUID", uo.uuid.0.as_ref())?;
            write_simple(writer, "Username", uo.username.0.as_ref())?;
            write_simple(writer, "Password", uo.password.0.as_ref())?;
            write_simple(writer, "Version", uo.version.0.as_ref())?;
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
        _attributes: &[xml::attribute::OwnedAttribute],
    ) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        match &path_pattern[..] {
            ["ChangeDUState", "Operations", "InstallOpStruct"] => {
                self.install_operations
                    .push(InstallOp::new("", "", "", "", ""));
            }
            ["ChangeDUState", "Operations", "UninstallOpStruct"] => {
                self.uninstall_operations.push(UninstallOp::new("", "", ""))
            }
            ["ChangeDUState", "Operations", "UpdateOpStruct"] => {
                self.update_operations
                    .push(UpdateOp::new("", "", "", "", ""));
            }
            _ => {}
        }
    }

    pub fn characters(&mut self, path: &[&str], characters: &str) {
        match *path {
            ["ChangeDUState", "CommandKey"] => self.command_key = characters.into(),
            ["ChangeDUState", "Operations", "InstallOpStruct", key] => {
                let last = self.install_operations.last_mut();
                if let Some(e) = last {
                    match key {
                        "URL" => e.url = characters.into(),
                        "UUID" => e.uuid = characters.into(),
                        "Username" => e.username = characters.into(),
                        "Password" => e.password = characters.into(),
                        "ExecutionEnvRef" => e.execution_env_ref = characters.into(),
                        _ => {}
                    }
                }
            }
            ["ChangeDUState", "Operations", "UninstallOpStruct", key] => {
                let last = self.uninstall_operations.last_mut();
                if let Some(e) = last {
                    match key {
                        "URL" => e.url = characters.into(),
                        "UUID" => e.uuid = characters.into(),
                        "ExecutionEnvRef" => e.execution_env_ref = characters.into(),
                        _ => {}
                    }
                }
            }
            ["ChangeDUState", "Operations", "UpdateOpStruct", key] => {
                let last = self.update_operations.last_mut();
                if let Some(e) = last {
                    match key {
                        "URL" => e.url = characters.into(),
                        "UUID" => e.uuid = characters.into(),
                        "Username" => e.username = characters.into(),
                        "Password" => e.password = characters.into(),
                        "Version" => e.version = characters.into(),
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for ChangeDUState {
    fn arbitrary(g: &mut Gen) -> Self {
        Self {
            command_key: XmlSafeString::arbitrary(g),
            install_operations: Vec::<InstallOp>::arbitrary(g),
            uninstall_operations: Vec::<UninstallOp>::arbitrary(g),
            update_operations: Vec::<UpdateOp>::arbitrary(g),
        }
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
