use dioxus::prelude::*;

pub static CallToAction: FC<()> = |(cx, _)| {
    cx.render(rsx!{
        section { class: "text-gray-400 bg-gray-800 body-font",
            div { class: "container px-5 py-12 mx-auto",
                div { class: "lg:w-2/3 flex flex-col sm:flex-row sm:items-center items-start mx-auto",
                    img { class: "h-12 mx-4" src: "https://rustacean.net/assets/rustacean-flat-gesture.png" }
                    h1 { class: "flex-grow sm:pr-16 text-2xl font-medium title-font text-white",
                        "Get started with building robust and portable apps with Dioxus."
                    }
                    button { class: "flex-shrink-0 text-white bg-indigo-500 border-0 py-2 px-8 focus:outline-none hover:bg-indigo-600 rounded text-lg mt-10 sm:mt-0",
                        "Get started"
                    }
                }
            }
        }        
    })
};
