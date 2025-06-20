//! Định nghĩa giao ước cho các module logic game có thể tái sử dụng để cắm vào `App`.

use crate::app::App;

/// Một trait để đóng gói logic và dữ liệu thành các đơn vị có thể tái sử dụng.
/// Các plugin được sử dụng để thêm systems, resources, và events vào `App`.
pub trait Plugin {
    /// Xây dựng plugin, thêm các thành phần của nó vào `App`.
    fn build(&self, app: &mut App);
}
