use dioxus::prelude::*;

#[inline_props]
#[allow(non_snake_case)]
pub fn NoteView<'a>(cx: Scope, contents: &'a str) -> Element {
	cx.render(
		rsx!(
			p {
				"Running a "
				b {
					"bold"
				}
				" test"
			}
		)
	)
}
