extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

mod methods;
mod native_script;
mod profiled;
mod variant;

#[proc_macro_attribute]
pub fn methods(meta: TokenStream, input: TokenStream) -> TokenStream {
    methods::derive_methods(meta, input)
}

/// Makes a function profiled in Godot's built-in profiler. This macro automatically
/// creates a tag using the name of the current module and the function by default.
///
/// This attribute may also be used on non-exported functions. If the GDNative API isn't
/// initialized when the function is called, the data will be ignored silently.
///
/// A custom tag can also be provided using the `tag` option.
///
/// See the `gdnative::nativescript::profiling` for a lower-level API to the profiler with
/// more control.
///
/// # Examples
///
/// ```ignore
/// mod foo {
///     // This function will show up as `foo/bar` under Script Functions.
///     #[gdnative::profiled]
///     fn bar() {
///         std::thread::sleep(std::time::Duration::from_millis(1));
///     }
/// }
/// ```
///
/// ```ignore
/// // This function will show up as `my_custom_tag` under Script Functions.
/// #[gdnative::profiled(tag = "my_custom_tag")]
/// fn baz() {
///     std::thread::sleep(std::time::Duration::from_millis(1));
/// }
/// ```
#[proc_macro_attribute]
pub fn profiled(meta: TokenStream, input: TokenStream) -> TokenStream {
    profiled::derive_profiled(meta, input)
}

/// Makes it possible to use a type as a NativeScript.
///
/// ## Required attributes
///
/// The following attributes are required on the type deriving `NativeClass`:
///
/// ### `#[inherit(gdnative::api::BaseClass)]`
///
/// Sets `gdnative::api::BaseClass` as the base class for the script. This *must* be
/// a type from the generated Godot API (that implements `GodotObject`). All `owner`
/// arguments of exported methods must be references (`TRef`, `Ref`, or `&`) to this
/// type.
///
/// Inheritance from other scripts, either in Rust or other languages, is
/// not supported.
///
/// ## Optional type attributes
///
/// Behavior of the derive macro can be customized using attribute on the type:
///
/// ### `#[user_data(gdnative::user_data::SomeWrapper<Self>)]`
///
/// Use the given type as the user-data wrapper. See the module-level docs on
/// `gdnative::user_data` for more information.
///
/// ### `#[register_with(path::to::function)]`
///
/// Use a custom function to register signals, properties or methods, in addition
/// to the one generated by `#[methods]`:
///
/// ```ignore
/// #[derive(NativeClass)]
/// #[inherit(Reference)]
/// #[register_with(my_register_function)]
/// struct Foo;
///
/// fn my_register_function(builder: &ClassBuilder<Foo>) {
///     builder.add_signal(Signal { name: "foo", args: &[] });
///     builder.add_property::<f32>("bar")
///         .with_getter(|_, _| 42.0)
///         .with_hint(FloatHint::Range(RangeHint::new(0.0, 100.0)))
///         .done();
/// }
/// ```
///
/// ### `#[no_constructor]`
///
/// Indicates that this type has no zero-argument constructor. Instances of such
/// scripts can only be created from Rust using `Instance::emplace`. `Instance::new`
/// or `ScriptName.new` from GDScript will result in panics at runtime.
///
/// See documentation on `Instance::emplace` for an example on how this can be used.
///
/// ## Optional field attributes
///
/// ### `#[property]`
///
/// Convenience attribute to register a field as a property. Possible arguments for
/// the attribute are:
///
/// - `path = "my_category/my_property_name"`
///
/// Puts the property under the `my_category` category and renames it to
/// `my_property_name` in the inspector and for GDScript.
///
/// - `default = 42.0`
///
/// Sets the default value *in the inspector* for this property. The setter is *not*
/// guaranteed to be called by the engine with the value.
///
/// - `before_get` / `after_get` / `before_set` / `after_set` `= "Self::hook_method"`
///
/// Call hook methods with `self` and `owner` before and/or after the generated property
/// accessors.
#[proc_macro_derive(
    NativeClass,
    attributes(
        inherit,
        export,
        opt,
        user_data,
        property,
        register_with,
        no_constructor
    )
)]
pub fn derive_native_class(input: TokenStream) -> TokenStream {
    native_script::derive_native_class(input)
}

#[proc_macro_derive(ToVariant, attributes(variant))]
pub fn derive_to_variant(input: TokenStream) -> TokenStream {
    variant::derive_to_variant(variant::ToVariantTrait::ToVariant, input)
}

#[proc_macro_derive(OwnedToVariant, attributes(variant))]
pub fn derive_owned_to_variant(input: TokenStream) -> TokenStream {
    variant::derive_to_variant(variant::ToVariantTrait::OwnedToVariant, input)
}

#[proc_macro_derive(FromVariant, attributes(variant))]
pub fn derive_from_variant(input: TokenStream) -> TokenStream {
    variant::derive_from_variant(input)
}
