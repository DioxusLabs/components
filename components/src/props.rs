/// Provides a prop struct with default props.
/// Usage:
/// ```rs
/// props!(MyStruct {
///     my_prop: i32,
///     
///     #[props(into)]
///     title: Option<String>,
/// });
///
/// // Outputs
/// #[derive(Props, PartialEq, Clone)]
/// pub struct AccordianItemProps {
///     // Default props
///     #[props(into)]
///     id: Option<String>,
///     #[props(into)]
///     class: Option<String>,
///     #[props(into)]
///     style: Option<String>,
///
///     // ... any other default props
///     // Your props
///     my_prop: i32,
///     
///     #[props(into)]
///     title: Option<String>,
/// }
/// ````
macro_rules! props {
    ($name:ident { $($(#[$attr:meta])? $field:ident : $type:ty),* $(,)? } ) => {
        #[derive(Props, PartialEq, Clone)]
        pub struct $name {
            #[props(into, optional, default = "".to_string())]
            id: String,

            #[props(into, optional, default = "".to_string())]
            class: String,

            #[props(into, optional, default = "".to_string())]
            style: String,

            $($(#[$attr])? $field: $type,)*
        }
    };
}
