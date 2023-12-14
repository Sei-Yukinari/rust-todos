// #[macro_export]
// macro_rules! impl_string_value_object {
//     ( $type:tt ) => {
//         impl $type {
//             pub fn new(id: i32) -> Result<Self, DomainError> {
//                 let object = Self { value: id };
//                 object.validate()?;
//                 Ok(object)
//             }
//             pub fn pase_int(&self) -> i32 {
//                 self.value
//             }
//             pub fn as_str(&self) -> &str {
//                 &self.value.to_string()
//             }
//             //
//             // pub fn into_string(self) -> String {
//             //     self.value.to_string()
//             // }
//         }
//     };
// }
