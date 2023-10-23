use bdk::bitcoin::blockdata::script::ScriptBuf as BdkScriptBuf;

/// A Bitcoin script.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Script(pub(crate) BdkScriptBuf);

impl Script {
    pub fn new(raw_output_script: Vec<u8>) -> Self {
        let script: BdkScriptBuf = BdkScriptBuf::from(raw_output_script);
        Script(script)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes()
    }
}

impl From<BdkScriptBuf> for Script {
    fn from(script: BdkScriptBuf) -> Self {
        Script(script)
    }
}
