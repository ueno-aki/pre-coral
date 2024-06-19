#[derive(Debug)]
pub enum ResponseStatus {
    Cancel = 1,
    Downloading = 2,
    DownloadingFinished = 3,
    ResourcePackStackFinished = 4,
}
