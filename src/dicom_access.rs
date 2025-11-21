use dicom::core::Tag;
use dicom::dictionary_std::StandardDataDictionary;
use dicom::object::{DefaultDicomObject, InMemDicomObject};

/// Small helper trait to pull string values from different DICOM object shapes.
pub trait ElementAccess {
    fn element_str(&self, tag: Tag) -> Option<String>;
    fn has_element(&self, tag: Tag) -> bool;
    fn transfer_syntax(&self) -> Option<String>;
}

impl ElementAccess for DefaultDicomObject {
    fn element_str(&self, tag: Tag) -> Option<String> {
        self.element(tag)
            .ok()
            .and_then(|e| e.to_str().ok())
            .map(|s| s.into_owned())
    }

    fn has_element(&self, tag: Tag) -> bool {
        self.element(tag).is_ok()
    }

    fn transfer_syntax(&self) -> Option<String> {
        Some(self.meta().transfer_syntax().to_string())
    }
}

impl ElementAccess for InMemDicomObject<StandardDataDictionary> {
    fn element_str(&self, tag: Tag) -> Option<String> {
        self.element(tag)
            .ok()
            .and_then(|e| e.to_str().ok())
            .map(|s| s.into_owned())
    }

    fn has_element(&self, tag: Tag) -> bool {
        self.element(tag).is_ok()
    }

    fn transfer_syntax(&self) -> Option<String> {
        None
    }
}
