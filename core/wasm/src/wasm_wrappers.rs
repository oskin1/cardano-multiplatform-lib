/// Various code-generation macros to help with WASM wrapper creation.
/// Includes many things that auto-generated by cddl-codegen so we can
/// use these with utility code that had to be hand-written.

/// Auto-declare From/AsRef conversions between rust and WASM wrappers
#[macro_export]
macro_rules! impl_wasm_conversions {
    ($rust:ty, $wasm:ty) => {
        impl From<$rust> for $wasm {
            fn from(native: $rust) -> Self {
                Self(native)
            }
        }

        #[allow(clippy::from_over_into)]
        impl Into<$rust> for $wasm {
            fn into(self) -> $rust {
                self.0
            }
        }

        impl AsRef<$rust> for $wasm {
            fn as_ref(&self) -> &$rust {
                &self.0
            }
        }
    };
}

/// This shouldn't be explicitly called - only via impl_wasm_list!()
/// We use this to get around restrictions in outer macros evaluating before inner macros
/// which breaks wasm_bindgen's parameter parsing resulting in FromWasmAbi on &T instead
/// See comment for impl_wasm_map_insert_get! for more context
#[macro_export]
macro_rules! impl_wasm_list_add {
    ($rust_elem_name:ty, $wasm_elem_name:ty, $wasm_list_name:ident, false) => {
        #[wasm_bindgen::prelude::wasm_bindgen]
        impl $wasm_list_name {
            pub fn add(&mut self, elem: &$wasm_elem_name) {
                self.0.push(elem.clone().into());
            }
        }
    };
    ($rust_elem_name:ty, $wasm_elem_name:ty, $wasm_list_name:ident, true) => {
        #[wasm_bindgen::prelude::wasm_bindgen]
        impl $wasm_list_name {
            pub fn add(&mut self, elem: $wasm_elem_name) {
                self.0.push(elem);
            }
        }
    };
}

// TODO: when/if concat_idents! leaves nightly/experimental
// make the default case for impl_wasm_list! / impl_wasm_map!
// not take in the name and instead derive it by appending List
// since that is what we do in virtually all cases anyway.

/// Convenience creator for generic WASM-exposable list
/// This is exactly as the ones created by cddl-codegen
/// so it is useful for utility functionality where those
/// wouldn't have been automatically generated.
#[macro_export]
macro_rules! impl_wasm_list {
    ($rust_elem_name:ty, $wasm_elem_name:ty, $wasm_list_name:ident) => {
        impl_wasm_list!(
            $rust_elem_name,
            $wasm_elem_name,
            $wasm_list_name,
            false,
            false
        );
    };
    ($rust_elem_name:ty, $wasm_elem_name:ty, $wasm_list_name:ident, $elem_wasm_abi:tt, $elem_copy:tt) => {
        #[wasm_bindgen::prelude::wasm_bindgen]
        #[derive(Debug, Clone)]
        pub struct $wasm_list_name(Vec<$rust_elem_name>);

        #[wasm_bindgen::prelude::wasm_bindgen]
        impl $wasm_list_name {
            pub fn new() -> Self {
                Self(Vec::new())
            }

            pub fn len(&self) -> usize {
                self.0.len()
            }

            pub fn get(&self, index: usize) -> $wasm_elem_name {
                $crate::wasm_val_into!(
                    $crate::wasm_val_clone!(self.0[index], $elem_copy),
                    $elem_wasm_abi
                )
            }
        }

        impl AsRef<[$rust_elem_name]> for $wasm_list_name {
            fn as_ref(&self) -> &[$rust_elem_name] {
                &self.0
            }
        }

        $crate::impl_wasm_list_add!(
            $rust_elem_name,
            $wasm_elem_name,
            $wasm_list_name,
            $elem_wasm_abi
        );

        $crate::impl_wasm_conversions!(Vec<$rust_elem_name>, $wasm_list_name);
    };
}

/// This shouldn't be explicitly called - only via impl_wasm_* macros here
/// expression, e is Copy
#[macro_export]
macro_rules! wasm_val_clone {
    ($e:expr, true) => {
        $e
    };
    ($e:expr, false) => {
        $e.clone()
    };
}

/// This shouldn't be explicitly called - only via impl_wasm_* macros here
/// expression, e is wasm-exposable
#[macro_export]
macro_rules! wasm_val_into {
    ($e:expr, true) => {
        $e
    };
    ($e:expr, false) => {
        $e.into()
    };
}

/// This shouldn't be explicitly called - only via impl_wasm_* macros here
/// expression, e is wasm-exposable
#[macro_export]
macro_rules! wasm_val_map_into {
    ($e:expr, true) => {
        $e
    };
    ($e:expr, false) => {
        $e.map(Into::into)
    };
}

/// This shouldn't be explicitly called - only via impl_wasm_* macros here
/// expression, e is Copy
#[macro_export]
macro_rules! wasm_copied_or_cloned {
    ($e:expr, true) => {
        $e.copied()
    };
    ($e:expr, false) => {
        $e.cloned()
    };
}

/// This shouldn't be explicitly called - only via impl_wasm_* macros here
/// This is here due to problems with AsRef<String> vs & in impl_wasm_get
/// expression, e is wasm-exposable
#[macro_export]
macro_rules! wasm_as_ref {
    ($e:expr, false) => {
        $e.as_ref()
    };
    ($e:expr, true) => {
        &$e
    };
}

/// This shouldn't be explicitly called - only via impl_wasm_* macros here
#[macro_export]
macro_rules! impl_wasm_map_get {
    ($self:ident, $key:ident, $key_wasm_abi:tt, $val_wasm_abi:tt, $val_copy:tt, $rust_key:ty) => {
        $crate::wasm_val_map_into!(
            $crate::wasm_copied_or_cloned!(
                $self
                    .0
                    .get::<$rust_key>($crate::wasm_as_ref!($key, $key_wasm_abi)),
                $val_copy
            ),
            $val_wasm_abi
        )
    };
}

/// This shouldn't be explicitly called - only via impl_wasm_* macros here
#[macro_export]
macro_rules! impl_wasm_map_insert {
    ($self:ident, $key:ident, $value:ident, $key_wasm_abi:tt, $val_wasm_abi:tt) => {
        $crate::wasm_val_map_into!(
            $self.0.insert(
                $crate::wasm_val_into!($crate::wasm_val_clone!($key, $key_wasm_abi), $key_wasm_abi),
                $crate::wasm_val_into!(
                    $crate::wasm_val_clone!($value, $val_wasm_abi),
                    $val_wasm_abi
                )
            ),
            $val_wasm_abi
        )
    };
}

/// This shouldn't be explicitly called - only via impl_wasm_map!()
/// We use this to get around restrictions in outer macros evaluating before inner macros
/// which breaks wasm_bindgen's parameter parsing resulting in FromWasmAbi on &T instead
/// of RefFromWasmAbi on T being used.
/// This happened when these were inlined directly in impl_wasm_map!() e.g.:
///
/// #[macro_export]
/// macro_rules! wasm_type_param {
///     ($wasm_type:ty, true) => {
///         $wasm_type
///     };
///     ($wasm_type:ty, false) => {
///         &$wasm_type
///     };
/// }
///
/// Then used within the impl of the wasm_bindgen map:
///
/// pub fn get(&self, key: $crate::wasm_type_param!($wasm_key, $key_wasm_abi)) -> Option<$wasm_value> {
/// $crate::wasm_option_return!(
///     self.0.get(key.as_ref()),
///     $val_wasm_abi)
/// }
/// pub fn insert(
/// &mut self,
///     key: $crate::wasm_type_param!($wasm_key, $key_wasm_abi),
///     value: $crate::wasm_type_param!($wasm_value, $val_wasm_abi)
/// ) -> Option<$wasm_value> {
/// self.0.insert(
///     $crate::wasm_val_clone!(key, $key_wasm_abi),
///     $crate::wasm_val_clone!(value, $val_wasm_abi))
/// }
#[macro_export]
macro_rules! impl_wasm_map_insert_get {
    ($rust_key:ty, $wasm_key:ty, $wasm_value:ty, $wasm_map_name:ident, false, false, $key_copy:tt, $val_copy:tt) => {
        #[wasm_bindgen::prelude::wasm_bindgen]
        impl $wasm_map_name {
            pub fn get(&self, key: &$wasm_key) -> Option<$wasm_value> {
                $crate::impl_wasm_map_get!(self, key, false, false, $val_copy, $rust_key)
            }

            pub fn insert(&mut self, key: &$wasm_key, value: &$wasm_value) -> Option<$wasm_value> {
                $crate::impl_wasm_map_insert!(self, key, value, false, false)
            }
        }
    };
    ($rust_key:ty, $wasm_key:ty, $wasm_value:ty, $wasm_map_name:ident, false, true, $key_copy:tt, $val_copy:tt) => {
        #[wasm_bindgen::prelude::wasm_bindgen]
        impl $wasm_map_name {
            pub fn get(&self, key: &$wasm_key) -> Option<$wasm_value> {
                $crate::impl_wasm_map_get!(self, key, false, true, $val_copy, $rust_key)
            }

            pub fn insert(&mut self, key: &$wasm_key, value: $wasm_value) -> Option<$wasm_value> {
                $crate::impl_wasm_map_insert!(self, key, value, false, true)
            }
        }
    };
    ($rust_key:ty, $wasm_key:ty, $wasm_value:ty, $wasm_map_name:ident, true, false, $key_copy:tt, $val_copy:tt) => {
        #[wasm_bindgen::prelude::wasm_bindgen]
        impl $wasm_map_name {
            pub fn get(&self, key: $wasm_key) -> Option<$wasm_value> {
                $crate::impl_wasm_map_get!(self, key, true, false, $val_copy, $rust_key)
            }

            pub fn insert(&mut self, key: $wasm_key, value: &$wasm_value) -> Option<$wasm_value> {
                $crate::impl_wasm_map_insert!(self, key, value, true, false)
            }
        }
    };
    ($rust_key:ty, $wasm_key:ty, $wasm_value:ty, $wasm_map_name:ident, true, true, $key_copy:tt, $val_copy:tt) => {
        #[wasm_bindgen::prelude::wasm_bindgen]
        impl $wasm_map_name {
            pub fn get(&self, key: $wasm_key) -> Option<$wasm_value> {
                $crate::impl_wasm_map_get!(self, key, true, true, $val_copy, $rust_key)
            }

            pub fn insert(&mut self, key: $wasm_key, value: $wasm_value) -> Option<$wasm_value> {
                $crate::impl_wasm_map_insert!(self, key, value, true, true)
            }
        }
    };
}

/// Useful for Byron/cip25/etc where we don't use OrderedHashMap
#[macro_export]
macro_rules! impl_wasm_map_btree {
    ($rust_key:ty, $rust_value:ty, $wasm_key:ty, $wasm_value:ty, $wasm_key_list:ty, $wasm_map_name:ident) => {
        impl_wasm_map_btree!($rust_key, $rust_value, $wasm_key, $wasm_value, $wasm_key_list, $wasm_map_name, false, false, false, false);
    };
    ($rust_key:ty, $rust_value:ty, $wasm_key:ty, $wasm_value:ty, $wasm_key_list:ty, $wasm_map_name:ident, $key_wasm_abi:tt, $val_wasm_abi:tt, $key_copy:tt, $val_copy:tt) => {
        $crate::impl_wasm_map!(
            $rust_key,
            $rust_value,
            $wasm_key,
            $wasm_value,
            $wasm_key_list,
            $wasm_map_name,
            $key_wasm_abi,
            $val_wasm_abi,
            $key_copy,
            $val_copy,
            std::collections::BTreeMap<$rust_key, $rust_value>
        );
    };
}

/// Convenience creator for generic WASM-exposable map
/// This is exactly as the ones created by cddl-codegen
/// so it is useful for utility functionality where those
/// wouldn't have been automatically generated.
#[macro_export]
macro_rules! impl_wasm_map {
    ($rust_key:ty, $rust_value:ty, $wasm_key:ty, $wasm_value:ty, $wasm_key_list:ty, $wasm_map_name:ident) => {
        impl_wasm_map!($rust_key, $rust_value, $wasm_key, $wasm_value, $wasm_key_list, $wasm_map_name, false, false, false, false);
    };
    ($rust_key:ty, $rust_value:ty, $wasm_key:ty, $wasm_value:ty, $wasm_key_list:ty, $wasm_map_name:ident, $key_wasm_abi:tt, $val_wasm_abi:tt, $key_copy:tt, $val_copy:tt) => {
        impl_wasm_map!(
            $rust_key,
            $rust_value,
            $wasm_key,
            $wasm_value,
            $wasm_key_list,
            $wasm_map_name,
            $key_wasm_abi,
            $val_wasm_abi,
            $key_copy,
            $val_copy,
            cml_core::ordered_hash_map::OrderedHashMap<$rust_key, $rust_value>
        );
    };
    ($rust_key:ty, $rust_value:ty, $wasm_key:ty, $wasm_value:ty, $wasm_key_list:ty, $wasm_map_name:ident, $key_wasm_abi:tt, $val_wasm_abi:tt, $key_copy:tt, $val_copy:tt, $map_type:ty) => {
        #[wasm_bindgen::prelude::wasm_bindgen]
        #[derive(Debug, Clone)]
        pub struct $wasm_map_name($map_type);

        $crate::impl_wasm_map_insert_get!(
            $rust_key,
            $wasm_key,
            $wasm_value,
            $wasm_map_name,
            $key_wasm_abi,
            $val_wasm_abi,
            $key_copy,
            $val_copy
        );

        #[wasm_bindgen::prelude::wasm_bindgen]
        impl $wasm_map_name {
            pub fn new() -> Self {
                Self(<$map_type>::new())
            }

            pub fn len(&self) -> usize {
                self.0.len()
            }

            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }

            pub fn keys(&self) -> $wasm_key_list {
                $crate::wasm_copied_or_cloned!(
                    self.0.keys(),
                    $key_copy)
                    .collect::<Vec<_>>()
                    .into()
            }
        }

        $crate::impl_wasm_conversions!(
            $map_type,
            $wasm_map_name
        );
    };
}

#[macro_export]
macro_rules! impl_wasm_cbor_json_api {
    ($wasm_name:ident) => {
        $crate::impl_wasm_cbor_api!($wasm_name);
        $crate::impl_wasm_json_api!($wasm_name);
    };
}

/// We use this instead of cml_core::impl_wasm_cbor_json_api for types that do not implement
/// cml's Serialize. e.g. CIP25/Byron just do cbor_event's due to not supporting preserve-encodings=true
/// All other methods are identical to impl_wasm_cbor_json_api though.
#[macro_export]
macro_rules! impl_wasm_cbor_json_api_cbor_event_serialize {
    ($wasm_name:ident) => {
        $crate::impl_wasm_cbor_event_serialize_api!($wasm_name);
        $crate::impl_wasm_json_api!($wasm_name);
    };
}

/// Implements to/from CBOR bytes API for WASM wrappers using CML's Serialize
/// i.e. it remembers encodings
#[macro_export]
macro_rules! impl_wasm_cbor_api {
    ($wasm_name:ident) => {
        #[wasm_bindgen::prelude::wasm_bindgen]
        impl $wasm_name {
            /**
             * Serialize this type to CBOR bytes
             * This type type supports encoding preservation so this will preserve round-trip CBOR formats.
             * If created from scratch the CBOR will be canonical.
             */
            pub fn to_cbor_bytes(&self) -> Vec<u8> {
                cml_core::serialization::Serialize::to_cbor_bytes(&self.0)
            }

            /**
             * Serialize this type to CBOR bytes using canonical CBOR encodings
             */
            pub fn to_canonical_cbor_bytes(&self) -> Vec<u8> {
                cml_core::serialization::Serialize::to_canonical_cbor_bytes(&self.0)
            }

            /**
             * Create this type from CBOR bytes
             */
            pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<$wasm_name, wasm_bindgen::JsError> {
                cml_core::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
                    .map(Self)
                    .map_err(|e| {
                        wasm_bindgen::JsError::new(&format!(
                            concat!(stringify!($wasm_name), "::from_bytes: {}"),
                            e
                        ))
                    })
            }

            /**
             * Serialize this type to CBOR bytes encoded as a hex string (useful for working with CIP30).
             * This type type supports encoding preservation so this will preserve round-trip CBOR formats.
             * If created from scratch the CBOR will be canonical.
             */
            pub fn to_cbor_hex(&self) -> String {
                hex::encode(self.to_cbor_bytes())
            }

            /**
             * Serialize this type to CBOR bytes using canonical CBOR encodings as hex bytes
             */
            pub fn to_canonical_cbor_hex(&self) -> String {
                hex::encode(self.to_canonical_cbor_bytes())
            }

            /**
             * Create this type from the CBOR bytes encoded as a hex string.
             * This is useful for interfacing with CIP30
             */
            pub fn from_cbor_hex(cbor_bytes: &str) -> Result<$wasm_name, wasm_bindgen::JsError> {
                hex::decode(cbor_bytes)
                    .map_err(|e| {
                        wasm_bindgen::JsError::new(&format!(
                            concat!(stringify!($wasm_name), "::from_cbor_hex: {}"),
                            e
                        ))
                    })
                    .and_then(|bytes| Self::from_cbor_bytes(&bytes))
            }
        }
    };
}

/// Implements to/from CBOR bytes API for WASM wrappers using cbor_event's Serialize
/// i.e. it does not remember encodings
#[macro_export]
macro_rules! impl_wasm_cbor_event_serialize_api {
    ($wasm_name:ident) => {
        #[wasm_bindgen::prelude::wasm_bindgen]
        impl $wasm_name {
            /**
             * Serialize this type to CBOR bytes.
             * This type does NOT support fine-tuned encoding options so this may or may not be
             * canonical CBOR and may or may not preserve round-trip encodings.
             */
            pub fn to_cbor_bytes(&self) -> Vec<u8> {
                cml_core::serialization::ToBytes::to_bytes(&self.0)
            }

            /**
             * Create this type from CBOR bytes
             */
            pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<$wasm_name, wasm_bindgen::JsError> {
                cml_core::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
                    .map(Self)
                    .map_err(|e| {
                        wasm_bindgen::JsError::new(&format!(
                            concat!(stringify!($wasm_name), "::from_cbor_bytes: {}"),
                            e
                        ))
                    })
            }

            /**
             * Serialize this type to CBOR bytes encoded as a hex string (useful for working with CIP30).
             * This type does NOT support fine-tuned encoding options so this may or may not be
             * canonical CBOR and may or may not preserve round-trip encodings.
             */
            pub fn to_cbor_hex(&self) -> String {
                hex::encode(self.to_cbor_bytes())
            }

            /**
             * Create this type from the CBOR bytes encoded as a hex string.
             * This is useful for interfacing with CIP30
             */
            pub fn from_cbor_hex(cbor_bytes: &str) -> Result<$wasm_name, wasm_bindgen::JsError> {
                hex::decode(cbor_bytes)
                    .map_err(|e| {
                        wasm_bindgen::JsError::new(&format!(
                            concat!(stringify!($wasm_name), "::from_cbor_hex: {}"),
                            e
                        ))
                    })
                    .and_then(|bytes| Self::from_cbor_bytes(&bytes))
            }
        }
    };
}

/// Implements the to/from JSON + JS object API for WASM wrappers
#[macro_export]
macro_rules! impl_wasm_json_api {
    ($wasm_name:ident) => {
        #[wasm_bindgen::prelude::wasm_bindgen]
        impl $wasm_name {
            pub fn to_json(&self) -> Result<String, wasm_bindgen::JsError> {
                serde_json::to_string_pretty(&self.0).map_err(|e| {
                    wasm_bindgen::JsError::new(&format!(
                        concat!(stringify!($wasm_name), "::to_json: {}"),
                        e
                    ))
                })
            }

            pub fn to_js_value(&self) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsError> {
                serde_wasm_bindgen::to_value(&self.0).map_err(|e| {
                    wasm_bindgen::JsError::new(&format!(
                        concat!(stringify!($wasm_name), "::to_js_value: {}"),
                        e
                    ))
                })
            }

            pub fn from_json(json: &str) -> Result<$wasm_name, wasm_bindgen::JsError> {
                serde_json::from_str(json).map(Self).map_err(|e| {
                    wasm_bindgen::JsError::new(&format!(
                        concat!(stringify!($wasm_name), "::from_json: {}"),
                        e
                    ))
                })
            }
        }
    };
}

#[macro_export]
macro_rules! impl_raw_bytes_api {
    ($rust:ty, $wasm:ident) => {
        #[wasm_bindgen]
        impl $wasm {
            /**
             * Direct raw bytes without any CBOR structure
             */
            pub fn to_raw_bytes(&self) -> Vec<u8> {
                use cml_core_wasm::RawBytesEncoding;
                self.0.to_raw_bytes().to_vec()
            }

            /**
             * Parse from the direct raw bytes, without any CBOR structure
             */
            pub fn from_raw_bytes(bytes: &[u8]) -> Result<$wasm, wasm_bindgen::JsError> {
                use cml_core_wasm::RawBytesEncoding;
                <$rust>::from_raw_bytes(bytes).map(Self).map_err(Into::into)
            }

            /**
             * Direct raw bytes without any CBOR structure, as a hex-encoded string
             */
            pub fn to_hex(&self) -> String {
                use cml_core_wasm::RawBytesEncoding;
                self.0.to_raw_hex()
            }

            /**
             * Parse from a hex string of the direct raw bytes, without any CBOR structure
             */
            pub fn from_hex(input: &str) -> Result<$wasm, wasm_bindgen::JsError> {
                use cml_core_wasm::RawBytesEncoding;
                <$rust>::from_raw_hex(input)
                    .map(Into::into)
                    .map(Self)
                    .map_err(Into::into)
            }
        }
    };
}
