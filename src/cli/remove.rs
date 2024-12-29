use clap::Args;

#[derive(Debug, Args)]
pub struct RemoveArgs {}

impl super::DoDoArgs for RemoveArgs {
    fn execute(&self) -> crate::Result<()> {
        todo!()
    }
}

// macro_rules! make_test {
//     ( $( $name:ident: $src:expr => $dst:expr),* ) => {

//         $(
//             #[test]
//             fn $name() {
//                 let src = String::from($src);
//                 let color : Color = src.into();
//                 assert_eq!($dst, color)
//             }
//         )*
//     }
// }
