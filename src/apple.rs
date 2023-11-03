use std::{io::Write, process::Command};

use anyhow::ensure;

use crate::split::split_by;

/// <https://github.com/niftycode/imessage_reader/blob/c021d1a4169fe5357d6f95e6dd4d615b378e88bf/imessage_reader/fetch_data.py#L42>
pub fn blob_to_text(blob_data: &[u8]) -> Option<String> {
    let blob_data = split_by(blob_data, b"NSString");

    let Some(blob_data) = blob_data.get(1) else {
        return None;
    };

    let blob_data = blob_data.get(5..).unwrap();
    let text = if blob_data[0] == 129 {
        let len = blob_data.get(1..3).unwrap();
        let len = u16::from_le_bytes([len[0], len[1]]) as usize;
        let blob_data = blob_data.get(3..).unwrap();
        let blob_data = blob_data.get(..len).unwrap();
        String::from_utf8(blob_data.to_vec()).unwrap()
    } else {
        let len = blob_data[0] as usize;
        let blob_data = blob_data.get(1..).unwrap();
        let blob_data = blob_data.get(..len).unwrap();

        // text = text[3:length + 3]
        String::from_utf8(blob_data.to_vec()).unwrap()
    };

    Some(text)
}

pub fn copy_to_clipboard(data: &str) -> anyhow::Result<()> {
    let mut pbcopy = Command::new("pbcopy")
        .stdin(std::process::Stdio::piped())
        .spawn()?;

    {
        let stdin = pbcopy.stdin.as_mut().unwrap();
        stdin.write_all(data.as_bytes())?;
    }

    let status = pbcopy.wait()?;
    ensure!(status.success(), "pbcopy failed");

    Ok(())
}
