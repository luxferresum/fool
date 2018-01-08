extern crate cursive;

mod buffer;

use buffer::{Buffer, ChangeType};

use cursive::Cursive;
use cursive::event::{Event, Key};
use cursive::traits::*;
use cursive::views::{Dialog, Panel, OnEventView, TextArea, TextView, BoxView, Menubar};
use cursive::menu::MenuTree;
use cursive::align::*;

use cursive::theme::*;

fn register_callbacks(siv: &mut Cursive) {

    // siv.add_global_callback(Key::Up, |_| eprintln!("ARROW UP"));
    // siv.add_global_callback(Key::Down, |_| eprintln!("ARROW DOWN"));

    siv.add_global_callback('s', |_| {
        eprintln!("Staging a file");
    });

    siv.add_global_callback('S', |_| {
        eprintln!("Staging ALL the file");
    });

    siv.add_global_callback('c', |_| {
        eprintln!("Creating a commit");
    });

    siv.add_global_callback('C', |_| {
        eprintln!("Ammend committing");
    });

    siv.add_global_callback('p', |_| {
        eprintln!("Pushing to origin");
    });
}

fn main() {
    let mut siv = Cursive::new();
    siv.load_theme_file("assets/style.toml").unwrap();

    // The main dialog will just have a textarea.
    // Its size expand automatically with the content.
    // siv.add_layer(
    //     Dialog::new()
    //         .title("Describe your issue")
    //         .padding((1, 1, 1, 0))
    //         .content(TextArea::new().with_id("text"))
    //         .button("Ok", Cursive::quit),
    // );

    let mut b = buffer::Buffer::new();
    b.add_untracked("src/control.rs".to_owned());
    b.add_unstaged("log.err".to_owned(), ChangeType::Deleted);
    b.add_unstaged("src/buffer.rs".to_owned(), ChangeType::Modified);
    b.add_unstaged("src/test.rs".to_owned(), ChangeType::Added);
    b.stage("src/main.rs".to_owned(), ChangeType::Modified);


    // " Local:    master ~/Projects/code/fool
    //  Head:     8ef7c41 Miep


    //  Changes:
    // ==> Modified   Cargo.lock
    //     Modified   Cargo.toml
    //     Modified   src/main.rs

    //  # Cheat Sheet
    //  #    s = stage file/section, S = stage all unstaged files
    //  #    c = commit, C = commit -a (add unstaged)
    //  #    P = push to upstream
    //     "

    let mut text_view = TextView::new(b.render());
    text_view.set_scrollable(false);

    let size = siv.screen_size();
    let view = BoxView::with_fixed_size((size.x - 8, size.y - 4), Panel::new(text_view));
    siv.add_layer(view);

    /* Register keybinding callbacks */
    register_callbacks(&mut siv);

    // The menubar is a list of (label, menu tree) pairs.
    siv.menubar()
        .add_subtree("Help", MenuTree::new())
        .add_subtree("Quit", MenuTree::new());

    siv.set_autohide_menu(false);
    siv.add_global_callback(Key::Esc, |s| s.select_menubar());
    
    siv.run();
}

// Dialog::new()
//     .title("Changes in the repo")
//     .padding((1, 1, 1, 1))
//     .h_align(HAlign::Center)
//     .content(TextArea::new().content("--------------------------------------------------------"))
//     .button("Ok", Cursive::quit),

// fn main() {
//     // Read some long text from a file.
//     let content = "Leverage agile frameworks to provide a robust
//     synopsis for high level overviews. Iterative approaches to corporate strategy foster collaborative thinking to further
// the overall value proposition. Organically grow the holistic world view of disruptive innovation via workplace diversity and empowerment.";
//     let mut siv = Cursive::new();

//     // We can quit by pressing q
//     siv.add_global_callback('q', |s| s.quit());

//     // The text is too long to fit on a line, so the view will wrap lines,
//     // and will adapt to the terminal size.
//     siv.add_layer(
//         Dialog::around(Panel::new(TextView::new(content)))
//             .h_align(HAlign::Center)
//             .button("Quit", |s| s.quit())
//             .full_screen(),
//     );
//     // Show a popup on top of the view.
//     siv.add_layer(Dialog::info(
//         "Try resizing the terminal!\n(Press 'q' to \
//          quit when you're done.)",
//     ));

//     siv.run();
// }
