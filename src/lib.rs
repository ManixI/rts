#[macro_export]
macro_rules! impl_getters {
    ($struct:ty, $($field:ident: $type:ty),*) => {
        paste::paste! {
            impl $struct {
                $(#[inline] pub fn [<get_ $field>](&self) -> $type { self.$field })*
            }
        }
    }
}

#[macro_export]
macro_rules! impl_setters {
    ($struct:ty, $($field:ident: $type:ty),*) => {
        paste::paste! {
            impl $struct {
                $(#[inline] pub fn [<set_ $field>](&mut self, new: $type) { self.$field = new })*
            }
        }
    }
}

#[macro_export]
macro_rules! impl_getters_setters {
    ($struct:ty, $($field:ident: $type:ty),*) => {
        paste::paste! {
            impl $struct {
                $(#[inline] pub fn [<get_ $field>](&self) -> $type { self.$field })*
                $(#[inline] pub fn [<set_ $field>](&mut self, new: $type) { self.$field = new })*
            }
        }
    };
}
