use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[derive(Clone, Debug, PartialEq)]
struct Project {
    id: usize,
    title: &'static str,
    tag: &'static str,
    category: &'static str,
    description: &'static str,
    tech_stack: Vec<&'static str>,
    stars: &'static str,
    latency: &'static str,
    link: &'static str,
    github: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
struct SkillGroup {
    category: &'static str,
    icon: &'static str,
    skills: Vec<(&'static str, &'static str, &'static str)>, // (Name, Level, Highlight)
}

#[component]
fn App() -> impl IntoView {
    let active_filter = RwSignal::new("all".to_string());
    let mobile_menu_open = RwSignal::new(false);
    let wasm_counter = RwSignal::new(100_000u64);
    let wasm_time_ms = RwSignal::new(Option::<f64>::None);
    let is_benchmarking = RwSignal::new(false);

    // Contact form state
    let contact_name = RwSignal::new(String::new());
    let contact_email = RwSignal::new(String::new());
    let contact_msg = RwSignal::new(String::new());
    let contact_submitted = RwSignal::new(false);

    let projects = vec![
        Project {
            id: 1,
            title: "Aura-Stream: Distributed Telemetry Engine",
            tag: "Rust & Systems",
            category: "rust",
            description: "High-throughput event streaming engine capable of processing 2.5M events/sec with sub-millisecond p99 latency using Tokio, SIMD, and custom ring buffers.",
            tech_stack: vec!["Rust", "Tokio", "SIMD", "WASM", "gRPC"],
            stars: "1.4k",
            latency: "< 0.4ms",
            link: "#",
            github: "https://github.com",
        },
        Project {
            id: 2,
            title: "KubeScale: Intelligent Autoscaler in Go",
            tag: "Go & Cloud",
            category: "go",
            description: "Kubernetes custom controller and operator that dynamically scales pods based on real-time traffic heuristics and predictive machine learning models.",
            tech_stack: vec!["Go", "Kubernetes", "CRDs", "Prometheus", "eBPF"],
            stars: "920",
            latency: "Zero cold-start",
            link: "#",
            github: "https://github.com",
        },
        Project {
            id: 3,
            title: "FinFlow: Enterprise Accounting Core",
            tag: "Python & Django",
            category: "python",
            description: "Robust multi-tenant financial transaction engine built with Django, PostgreSQL partition tables, and Celery background reconciliation queues.",
            tech_stack: vec!["Python", "Django", "PostgreSQL", "Celery", "Redis"],
            stars: "640",
            latency: "ACID Safe",
            link: "#",
            github: "https://github.com",
        },
        Project {
            id: 4,
            title: "LeptosPulse: Reactive Cloud Monitor",
            tag: "Rust & Leptos",
            category: "rust",
            description: "Full-stack client-side rendered dashboard built in pure Rust and Leptos. Zero JavaScript dependencies, blazing 60fps SVG rendering.",
            tech_stack: vec!["Rust", "Leptos", "WebAssembly", "Trunk", "Tailwind"],
            stars: "1.1k",
            latency: "60 FPS",
            link: "#",
            github: "https://github.com",
        },
        Project {
            id: 5,
            title: "HyperMesh: Microservice Service Mesh",
            tag: "Go & Networking",
            category: "go",
            description: "Ultra lightweight L7 proxy and mTLS security mesh for bare-metal clusters, written in pure Go with zero memory allocations on the hot path.",
            tech_stack: vec!["Go", "mTLS", "HTTP/3", "gRPC", "Envoy API"],
            stars: "830",
            latency: "12μs overhead",
            link: "#",
            github: "https://github.com",
        },
        Project {
            id: 6,
            title: "NeuralDoc: Semantic Search Platform",
            tag: "Python & AI",
            category: "python",
            description: "Enterprise knowledge base search engine utilizing FastAPI, pgvector, and transformer embeddings to deliver sub-10ms semantic document retrieval.",
            tech_stack: vec!["Python", "FastAPI", "pgvector", "LangChain", "Docker"],
            stars: "580",
            latency: "< 9ms",
            link: "#",
            github: "https://github.com",
        },
    ];

    let skill_groups = vec![
        SkillGroup {
            category: "Rust Ecosystem",
            icon: "🦀",
            skills: vec![
                ("Rust Core & Memory Model", "Advanced", "Ownership, Borrow Checker, Lifetimes"),
                ("WebAssembly & Leptos", "Expert", "Fine-grained reactivity, Trunk, CSR/SSR"),
                ("Tokio & Async I/O", "Advanced", "Multi-threading, Channels, Actors"),
                ("Axum & Actix-web", "Expert", "High-throughput REST & WebSocket APIs"),
            ],
        },
        SkillGroup {
            category: "Go Systems",
            icon: "🐹",
            skills: vec![
                ("Go Concurrency & Internals", "Expert", "Goroutines, Channels, Memory Profiling"),
                ("Microservices & gRPC", "Advanced", "Protobuf, Service Discovery, Observability"),
                ("Kubernetes & Cloud Native", "Proficient", "Custom Controllers, Client-go, Helm"),
                ("Gin, Fiber & Standard Library", "Expert", "Production APIs, Zero-alloc code"),
            ],
        },
        SkillGroup {
            category: "Python & Data / Backend",
            icon: "🐍",
            skills: vec![
                ("Django & Django REST", "Expert", "ORM optimization, Multi-tenancy, Auth"),
                ("FastAPI & Pydantic", "Expert", "Asynchronous endpoints, OpenAPI, typing"),
                ("Celery & Task Pipelines", "Advanced", "Distributed queuing, cron, Redis broker"),
                ("Testing & CI/CD", "Advanced", "Pytest, Ruff, Mypy, GitHub Actions"),
            ],
        },
        SkillGroup {
            category: "Databases & Infrastructure",
            icon: "⚡",
            skills: vec![
                ("PostgreSQL & pgvector", "Advanced", "Indexing, partitioning, ACID transactions"),
                ("Redis & In-Memory Stores", "Advanced", "Caching, pub/sub, rate limiting"),
                ("Docker & Containerization", "Expert", "Multi-stage scratch builds, security"),
                ("Linux & Shell Scripting", "Expert", "Ubuntu, Bash/Zsh, performance tuning"),
            ],
        },
    ];

    // Benchmark action
    let run_benchmark = move |_| {
        is_benchmarking.set(true);
        let n = wasm_counter.get();
        
        let start = web_sys::window()
            .and_then(|w| w.performance())
            .map(|p| p.now())
            .unwrap_or(0.0);

        // CPU computation in pure WebAssembly
        let mut sum: u64 = 0;
        let mut a: u64 = 0;
        let mut b: u64 = 1;
        for _ in 0..n {
            sum = sum.wrapping_add(a);
            let next = a.wrapping_add(b);
            a = b;
            b = next;
        }

        let end = web_sys::window()
            .and_then(|w| w.performance())
            .map(|p| p.now())
            .unwrap_or(0.0);

        let elapsed = (end - start).max(0.01);
        wasm_time_ms.set(Some(elapsed));
        is_benchmarking.set(false);
    };

    view! {
        <div class="min-h-screen bg-[#0a0f1d] text-slate-100 flex flex-col font-sans selection:bg-orange-500 selection:text-white">
            // Navigation Bar
            <nav class="sticky top-0 z-50 backdrop-blur-md bg-[#0a0f1d]/80 border-b border-slate-800/80 px-6 py-4 transition-all">
                <div class="max-w-7xl mx-auto flex items-center justify-between">
                    <a href="#" class="flex items-center space-x-3 group">
                        <div class="w-10 h-10 rounded-xl bg-gradient-to-tr from-orange-600 to-amber-500 flex items-center justify-center font-mono font-bold text-lg text-white shadow-lg shadow-orange-500/20 group-hover:scale-105 transition-transform">
                            "🦀"
                        </div>
                        <div class="flex flex-col">
                            <span class="font-bold text-lg tracking-tight group-hover:text-orange-400 transition-colors">
                                "Khairul"<span class="text-orange-500">".rs"</span>
                            </span>
                            <span class="text-xs text-slate-400 font-mono">"Rust • Leptos • WebAssembly"</span>
                        </div>
                    </a>

                    // Desktop menu
                    <div class="hidden md:flex items-center space-x-8 text-sm font-medium text-slate-300">
                        <a href="#about" class="hover:text-orange-400 transition-colors">"About"</a>
                        <a href="#benchmark" class="hover:text-orange-400 transition-colors flex items-center gap-1.5">
                            <span class="w-2 h-2 rounded-full bg-orange-400 animate-ping"></span>
                            "WASM Benchmark"
                        </a>
                        <a href="#skills" class="hover:text-orange-400 transition-colors">"Skills"</a>
                        <a href="#projects" class="hover:text-orange-400 transition-colors">"Projects"</a>
                        <a href="#experience" class="hover:text-orange-400 transition-colors">"Experience"</a>
                        <a href="#contact" class="hover:text-orange-400 transition-colors">"Contact"</a>
                    </div>

                    <div class="hidden md:flex items-center space-x-4">
                        <a
                            href="https://github.com"
                            target="_blank"
                            class="px-4 py-2 rounded-lg border border-slate-700 hover:border-orange-500/50 bg-slate-900/60 hover:bg-slate-800 text-sm font-medium transition-all flex items-center gap-2"
                        >
                            <span>"GitHub"</span>
                            <span class="text-xs bg-slate-800 px-1.5 py-0.5 rounded text-orange-400 font-mono">"★"</span>
                        </a>
                        <a
                            href="#contact"
                            class="px-5 py-2 rounded-lg bg-gradient-to-r from-orange-600 to-amber-600 hover:from-orange-500 hover:to-amber-500 text-white font-medium text-sm shadow-lg shadow-orange-600/30 hover:shadow-orange-600/50 transition-all"
                        >
                            "Hire Me"
                        </a>
                    </div>

                    // Mobile Menu Toggle Button
                    <button
                        on:click=move |_| mobile_menu_open.update(|v| *v = !*v)
                        class="md:hidden p-2 rounded-lg bg-slate-800 text-slate-300 hover:text-white focus:outline-none"
                    >
                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16m-7 6h7"></path>
                        </svg>
                    </button>
                </div>

                // Mobile dropdown
                {move || if mobile_menu_open.get() {
                    view! {
                        <div class="md:hidden mt-4 pt-4 border-t border-slate-800 flex flex-col space-y-3 text-sm">
                            <a href="#about" on:click=move |_| mobile_menu_open.set(false) class="px-2 py-1 text-slate-300 hover:text-orange-400">"About"</a>
                            <a href="#benchmark" on:click=move |_| mobile_menu_open.set(false) class="px-2 py-1 text-slate-300 hover:text-orange-400">"WASM Benchmark"</a>
                            <a href="#skills" on:click=move |_| mobile_menu_open.set(false) class="px-2 py-1 text-slate-300 hover:text-orange-400">"Skills"</a>
                            <a href="#projects" on:click=move |_| mobile_menu_open.set(false) class="px-2 py-1 text-slate-300 hover:text-orange-400">"Projects"</a>
                            <a href="#experience" on:click=move |_| mobile_menu_open.set(false) class="px-2 py-1 text-slate-300 hover:text-orange-400">"Experience"</a>
                            <a href="#contact" on:click=move |_| mobile_menu_open.set(false) class="px-2 py-1 text-slate-300 hover:text-orange-400">"Contact"</a>
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }}
            </nav>

            // Hero Section
            <section id="about" class="relative pt-20 pb-28 px-6 overflow-hidden">
                // Background radial glows
                <div class="absolute top-1/4 left-1/2 -translate-x-1/2 w-[700px] h-[500px] bg-orange-600/10 rounded-full blur-[140px] pointer-events-none -z-10"></div>
                <div class="absolute top-1/3 right-10 w-[400px] h-[400px] bg-amber-500/10 rounded-full blur-[120px] pointer-events-none -z-10"></div>

                <div class="max-w-7xl mx-auto grid grid-cols-1 lg:grid-cols-12 gap-12 items-center">
                    // Left Column: Intro
                    <div class="lg:col-span-7 space-y-6">
                        <div class="inline-flex items-center gap-2.5 px-3.5 py-1.5 rounded-full bg-slate-900 border border-slate-700/80 text-xs font-mono text-orange-400 shadow-sm">
                            <span class="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"></span>
                            "Available for Software Engineering Roles"
                        </div>

                        <h1 class="text-4xl sm:text-6xl font-extrabold tracking-tight leading-[1.15] text-white">
                            "Building high-speed systems with "
                            <span class="text-transparent bg-clip-text bg-gradient-to-r from-orange-400 via-amber-300 to-orange-500">
                                "Rust, Go & Python"
                            </span>
                        </h1>

                        <p class="text-lg text-slate-300 max-w-2xl leading-relaxed">
                            "Hi, I'm Khairul. I specialize in memory-safe systems, low-latency microservices, and reactive full-stack web applications. This entire portfolio is compiled to "
                            <span class="font-mono text-orange-400 bg-orange-950/40 px-1.5 py-0.5 rounded border border-orange-800/40">"WebAssembly"</span>
                            " using "
                            <span class="font-semibold text-white">"Rust & Leptos"</span>
                            " for near-native in-browser performance."
                        </p>

                        <div class="flex flex-wrap gap-4 pt-2">
                            <a
                                href="#projects"
                                class="px-6 py-3.5 rounded-xl bg-gradient-to-r from-orange-600 to-amber-600 hover:from-orange-500 hover:to-amber-500 text-white font-semibold shadow-xl shadow-orange-600/25 hover:shadow-orange-600/40 transition-all flex items-center gap-2 group"
                            >
                                <span>"Explore Projects"</span>
                                <span class="group-hover:translate-x-1 transition-transform">"→"</span>
                            </a>
                            <a
                                href="#benchmark"
                                class="px-6 py-3.5 rounded-xl bg-slate-900 hover:bg-slate-800 border border-slate-700 text-slate-200 font-semibold transition-all flex items-center gap-2"
                            >
                                <span>"Run WASM Benchmark"</span>
                                <span>"⚡"</span>
                            </a>
                        </div>

                        // Tech tags
                        <div class="pt-6 border-t border-slate-800/80 flex items-center gap-3 text-xs text-slate-400 font-mono">
                            <span class="text-slate-300 font-bold">"Core:"</span>
                            <span class="bg-slate-900 border border-slate-800 px-2 py-1 rounded text-orange-300">"🦀 Rust 1.100"</span>
                            <span class="bg-slate-900 border border-slate-800 px-2 py-1 rounded text-cyan-300">"🐹 Go 1.27"</span>
                            <span class="bg-slate-900 border border-slate-800 px-2 py-1 rounded text-yellow-300">"🐍 Python 3.14 (Django)"</span>
                            <span class="bg-slate-900 border border-slate-800 px-2 py-1 rounded text-purple-300">"⚡ Leptos 0.7"</span>
                        </div>
                    </div>

                    // Right Column: Terminal Preview Card
                    <div class="lg:col-span-5">
                        <div class="rounded-2xl bg-slate-900/90 border border-slate-800 shadow-2xl p-5 font-mono text-xs backdrop-blur-xl relative group hover:border-orange-500/40 transition-all">
                            // Window Header
                            <div class="flex items-center justify-between pb-4 border-b border-slate-800">
                                <div class="flex items-center space-x-2">
                                    <div class="w-3 h-3 rounded-full bg-rose-500/80"></div>
                                    <div class="w-3 h-3 rounded-full bg-amber-500/80"></div>
                                    <div class="w-3 h-3 rounded-full bg-emerald-500/80"></div>
                                </div>
                                <span class="text-slate-400 text-[11px]">"main.rs (Leptos CSR)"</span>
                                <span class="text-[10px] text-orange-400 font-bold bg-orange-950/60 px-2 py-0.5 rounded border border-orange-800/40">"wasm32"</span>
                            </div>

                            // Code Snippet
                            <div class="mt-4 space-y-1.5 text-slate-300 overflow-x-auto">
                                <p class="text-slate-400">"// 🦀 Leptos Reactive Component"</p>
                                <p><span class="text-purple-400">"use"</span>" leptos::prelude::*;"</p>
                                <p class="pt-1"><span class="text-orange-400">"#[component]"</span></p>
                                <p><span class="text-blue-400">"pub fn"</span> <span class="text-amber-300">"PortfolioApp"</span>"() -> "<span class="text-emerald-400">"impl IntoView"</span>" {"</p>
                                <p class="pl-4"><span class="text-blue-400">"let"</span>" visitor_status = "<span class="text-amber-300">"RwSignal::new"</span><span class="text-orange-300">"(\"Welcome!\")"</span>";"</p>
                                <p class="pl-4 pt-1"><span class="text-purple-400">"view!"</span>" {"</p>
                                <p class="pl-8 text-cyan-300">"<div class=\"hero\">"</p>
                                <p class="pl-12 text-emerald-300">"<h1>{move || visitor_status.get()}</h1>"</p>
                                <p class="pl-12 text-amber-300">"<WasmBenchmark iterations=100_000 />"</p>
                                <p class="pl-8 text-cyan-300">"</div>"</p>
                                <p class="pl-4">"}"</p>
                                <p>"}"</p>
                            </div>

                            // Live compiler output badge
                            <div class="mt-5 p-3 rounded-lg bg-[#070b14] border border-emerald-900/50 flex items-center justify-between">
                                <div class="flex items-center gap-2 text-emerald-400">
                                    <span class="w-2 h-2 rounded-full bg-emerald-500 animate-pulse"></span>
                                    <span class="font-bold">"Trunk Wasm Compilation"</span>
                                </div>
                                <span class="text-slate-400">"Optimized & Hot"</span>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // Interactive WASM Benchmark Playground
            <section id="benchmark" class="py-16 px-6 bg-slate-900/50 border-y border-slate-800/80">
                <div class="max-w-5xl mx-auto">
                    <div class="text-center space-y-2 mb-10">
                        <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-orange-950/60 border border-orange-800/40 text-xs font-mono text-orange-400">
                            "⚡ Live Client-Side WebAssembly Demo"
                        </div>
                        <h2 class="text-3xl font-bold tracking-tight text-white">"Rust in Your Browser"</h2>
                        <p class="text-slate-400 text-sm max-w-xl mx-auto">
                            "This interactive test executes directly inside your browser's WebAssembly sandbox. Adjust the loop count and witness the raw execution speed of Rust."
                        </p>
                    </div>

                    <div class="p-8 rounded-2xl bg-slate-900 border border-slate-800 shadow-xl max-w-2xl mx-auto space-y-6">
                        <div class="space-y-2">
                            <div class="flex justify-between items-center text-sm">
                                <span class="font-medium text-slate-300">"Computation Iterations:"</span>
                                <span class="font-mono text-orange-400 font-bold text-base">
                                    {move || format!("{:#} cycles", wasm_counter.get())}
                                </span>
                            </div>
                            <input
                                type="range"
                                min="10000"
                                max="1000000"
                                step="10000"
                                prop:value=move || wasm_counter.get()
                                on:input=move |ev| {
                                    if let Ok(val) = event_target_value(&ev).parse::<u64>() {
                                        wasm_counter.set(val);
                                    }
                                }
                                class="w-full accent-orange-500 h-2 bg-slate-800 rounded-lg cursor-pointer"
                            />
                        </div>

                        <div class="flex flex-col sm:flex-row items-center gap-4 justify-between pt-2">
                            <button
                                on:click=run_benchmark
                                disabled=move || is_benchmarking.get()
                                class="w-full sm:w-auto px-6 py-3 rounded-xl bg-orange-600 hover:bg-orange-500 active:scale-95 text-white font-semibold transition-all shadow-lg shadow-orange-600/30 flex items-center justify-center gap-2"
                            >
                                <span>"Execute WASM Kernel"</span>
                                <span>"⚡"</span>
                            </button>

                            <div class="w-full sm:w-auto text-right">
                                {move || match wasm_time_ms.get() {
                                    Some(ms) => view! {
                                        <div class="p-2.5 rounded-lg bg-emerald-950/60 border border-emerald-700/50 text-emerald-300 text-sm font-mono flex items-center gap-2">
                                            <span class="w-2 h-2 rounded-full bg-emerald-400"></span>
                                            <span>"Executed in " <strong class="text-white font-bold">{format!("{:.2} ms", ms)}</strong></span>
                                        </div>
                                    }.into_any(),
                                    None => view! {
                                        <span class="text-xs text-slate-400 font-mono">"Click execute to measure time"</span>
                                    }.into_any()
                                }}
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // Stats Section
            <section class="py-14 px-6 border-b border-slate-800/80">
                <div class="max-w-7xl mx-auto grid grid-cols-2 md:grid-cols-4 gap-6">
                    <div class="p-6 rounded-2xl bg-slate-900/60 border border-slate-800 text-center space-y-1">
                        <div class="text-3xl sm:text-4xl font-extrabold text-orange-400 font-mono">"5+"</div>
                        <div class="text-xs uppercase tracking-wider text-slate-400 font-medium">"Years Engineering"</div>
                    </div>
                    <div class="p-6 rounded-2xl bg-slate-900/60 border border-slate-800 text-center space-y-1">
                        <div class="text-3xl sm:text-4xl font-extrabold text-amber-400 font-mono">"30+"</div>
                        <div class="text-xs uppercase tracking-wider text-slate-400 font-medium">"Production Crates & Repos"</div>
                    </div>
                    <div class="p-6 rounded-2xl bg-slate-900/60 border border-slate-800 text-center space-y-1">
                        <div class="text-3xl sm:text-4xl font-extrabold text-emerald-400 font-mono">"99.99%"</div>
                        <div class="text-xs uppercase tracking-wider text-slate-400 font-medium">"Service Uptime"</div>
                    </div>
                    <div class="p-6 rounded-2xl bg-slate-900/60 border border-slate-800 text-center space-y-1">
                        <div class="text-3xl sm:text-4xl font-extrabold text-cyan-400 font-mono">"< 1ms"</div>
                        <div class="text-xs uppercase tracking-wider text-slate-400 font-medium">"Average p99 Latency"</div>
                    </div>
                </div>
            </section>

            // Skills Section
            <section id="skills" class="py-24 px-6">
                <div class="max-w-7xl mx-auto space-y-12">
                    <div class="text-center space-y-3">
                        <h2 class="text-3xl sm:text-4xl font-bold tracking-tight text-white">"Technical Arsenal"</h2>
                        <p class="text-slate-400 text-base max-w-xl mx-auto">
                            "Deep expertise in systems programming, high-concurrency microservices, and modern web application development."
                        </p>
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                        {skill_groups.into_iter().map(|group| {
                            view! {
                                <div class="p-7 rounded-2xl bg-slate-900/70 border border-slate-800 hover:border-orange-500/30 transition-all space-y-5">
                                    <div class="flex items-center gap-3">
                                        <span class="text-2xl">{group.icon}</span>
                                        <h3 class="text-xl font-bold text-white tracking-tight">{group.category}</h3>
                                    </div>

                                    <div class="space-y-4">
                                        {group.skills.into_iter().map(|(name, level, highlight)| {
                                            view! {
                                                <div class="space-y-1 pb-3 border-b border-slate-800/60 last:border-0 last:pb-0">
                                                    <div class="flex justify-between items-center text-sm">
                                                        <span class="font-semibold text-slate-200">{name}</span>
                                                        <span class="text-[11px] font-mono px-2 py-0.5 rounded bg-orange-950/60 text-orange-400 border border-orange-900/40">{level}</span>
                                                    </div>
                                                    <p class="text-xs text-slate-400 font-mono">{highlight}</p>
                                                </div>
                                            }
                                        }).collect_view()}
                                    </div>
                                </div>
                            }
                        }).collect_view()}
                    </div>
                </div>
            </section>

            // Projects Section
            <section id="projects" class="py-24 px-6 bg-slate-900/40 border-t border-slate-800/80">
                <div class="max-w-7xl mx-auto space-y-12">
                    <div class="flex flex-col md:flex-row md:items-end justify-between gap-6">
                        <div class="space-y-2">
                            <h2 class="text-3xl sm:text-4xl font-bold tracking-tight text-white">"Featured Projects"</h2>
                            <p class="text-slate-400 text-sm max-w-xl">
                                "Production-grade systems, microservices, and full-stack solutions built for scale and reliability."
                            </p>
                        </div>

                        // Filter Tabs
                        <div class="flex flex-wrap gap-2 bg-slate-900 p-1.5 rounded-xl border border-slate-800 text-xs font-medium">
                            {vec![
                                ("all", "All Projects"),
                                ("rust", "Rust & WASM"),
                                ("go", "Go Services"),
                                ("python", "Python & Django"),
                            ].into_iter().map(|(key, label)| {
                                let key_clone = key.to_string();
                                let is_active = move || active_filter.get() == key;
                                view! {
                                    <button
                                        on:click=move |_| active_filter.set(key_clone.clone())
                                        class=move || {
                                            if is_active() {
                                                "px-3.5 py-1.5 rounded-lg bg-orange-600 text-white font-semibold transition-all shadow-md shadow-orange-600/30"
                                            } else {
                                                "px-3.5 py-1.5 rounded-lg text-slate-400 hover:text-white transition-colors"
                                            }
                                        }
                                    >
                                        {label}
                                    </button>
                                }
                            }).collect_view()}
                        </div>
                    </div>

                    // Project Cards Grid
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                        {move || {
                            let filter = active_filter.get();
                            projects.iter().filter(move |p| filter == "all" || p.category == filter).map(|p| {
                                view! {
                                    <div class="rounded-2xl bg-slate-900 border border-slate-800 hover:border-orange-500/40 p-6 flex flex-col justify-between transition-all duration-300 hover:-translate-y-1 hover:shadow-2xl hover:shadow-orange-950/20 group">
                                        <div class="space-y-4">
                                            <div class="flex items-center justify-between">
                                                <span class="text-[11px] font-mono px-2.5 py-1 rounded-full bg-slate-800 text-orange-400 border border-slate-700">
                                                    {p.tag}
                                                </span>
                                                <span class="text-xs text-slate-400 font-mono">"★ " {p.stars}</span>
                                            </div>

                                            <h3 class="text-lg font-bold text-white group-hover:text-orange-400 transition-colors">
                                                {p.title}
                                            </h3>

                                            <p class="text-slate-400 text-sm leading-relaxed">
                                                {p.description}
                                            </p>

                                            <div class="flex flex-wrap gap-1.5 pt-2">
                                                {p.tech_stack.iter().map(|t| {
                                                    let tech: &'static str = *t;
                                                    view! {
                                                        <span class="text-[11px] font-mono px-2 py-0.5 rounded bg-slate-800/80 text-slate-300">
                                                            {tech}
                                                        </span>
                                                    }
                                                }).collect_view()}
                                            </div>
                                        </div>

                                        <div class="pt-6 mt-6 border-t border-slate-800 flex items-center justify-between text-xs">
                                            <span class="text-emerald-400 font-mono font-medium">"⚡ " {p.latency}</span>
                                            <a
                                                href=p.github
                                                target="_blank"
                                                class="font-semibold text-orange-400 hover:text-orange-300 transition-colors flex items-center gap-1"
                                            >
                                                <span>"View Source"</span>
                                                <span>"→"</span>
                                            </a>
                                        </div>
                                    </div>
                                }
                            }).collect_view()
                        }}
                    </div>
                </div>
            </section>

            // Experience Section
            <section id="experience" class="py-24 px-6">
                <div class="max-w-4xl mx-auto space-y-12">
                    <div class="text-center space-y-3">
                        <h2 class="text-3xl sm:text-4xl font-bold tracking-tight text-white">"Career & Experience"</h2>
                        <p class="text-slate-400 text-sm">"Proven track record of architecting mission-critical platforms."</p>
                    </div>

                    <div class="space-y-8 relative before:absolute before:inset-0 before:left-4 before:w-0.5 before:bg-slate-800">
                        // Item 1
                        <div class="relative pl-10 space-y-2">
                            <div class="absolute left-2.5 top-1.5 -translate-x-1/2 w-4 h-4 rounded-full bg-orange-500 border-4 border-slate-900"></div>
                            <div class="flex flex-wrap items-center justify-between gap-2">
                                <h3 class="text-lg font-bold text-white">"Senior Systems & Backend Engineer"</h3>
                                <span class="text-xs font-mono px-2 py-0.5 rounded bg-slate-800 text-slate-300">"2023 — Present"</span>
                            </div>
                            <p class="text-xs font-mono text-orange-400">"Distributed Infrastructure & Cloud Systems"</p>
                            <p class="text-sm text-slate-400 leading-relaxed">
                                "Led migration of legacy monolithic services into Rust and Go microservices. Reduced p99 latency from 180ms to 12ms and saved 45% on compute infrastructure costs."
                            </p>
                        </div>

                        // Item 2
                        <div class="relative pl-10 space-y-2">
                            <div class="absolute left-2.5 top-1.5 -translate-x-1/2 w-4 h-4 rounded-full bg-amber-500 border-4 border-slate-900"></div>
                            <div class="flex flex-wrap items-center justify-between gap-2">
                                <h3 class="text-lg font-bold text-white">"Full-Stack Python & Web Engineer"</h3>
                                <span class="text-xs font-mono px-2 py-0.5 rounded bg-slate-800 text-slate-300">"2021 — 2023"</span>
                            </div>
                            <p class="text-xs font-mono text-amber-400">"FinTech & Enterprise Solutions"</p>
                            <p class="text-sm text-slate-400 leading-relaxed">
                                "Architected scalable multi-tenant Django applications and Celery pipelines serving over 1M active users. Designed REST APIs and automated CI/CD deployments."
                            </p>
                        </div>
                    </div>
                </div>
            </section>

            // Contact Section
            <section id="contact" class="py-24 px-6 bg-slate-900/60 border-t border-slate-800">
                <div class="max-w-3xl mx-auto space-y-10">
                    <div class="text-center space-y-3">
                        <h2 class="text-3xl sm:text-4xl font-bold tracking-tight text-white">"Let's Build Something Great"</h2>
                        <p class="text-slate-400 text-sm">
                            "Interested in collaborating or hiring for your team? Send a message directly."
                        </p>
                    </div>

                    <div class="p-8 rounded-2xl bg-slate-900 border border-slate-800 shadow-2xl space-y-5">
                        {move || if contact_submitted.get() {
                            view! {
                                <div class="p-6 rounded-xl bg-emerald-950/60 border border-emerald-800/60 text-center space-y-2">
                                    <div class="text-3xl">"🎉"</div>
                                    <h4 class="text-lg font-bold text-emerald-300">"Message Received!"</h4>
                                    <p class="text-xs text-slate-300">
                                        "Thank you, " <strong class="text-white">{contact_name.get()}</strong>"! I will get back to you shortly."
                                    </p>
                                    <button
                                        on:click=move |_| contact_submitted.set(false)
                                        class="mt-3 text-xs text-orange-400 hover:underline"
                                    >
                                        "Send another message"
                                    </button>
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <div class="space-y-4">
                                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                                        <div class="space-y-1.5">
                                            <label class="text-xs font-medium text-slate-300">"Your Name"</label>
                                            <input
                                                type="text"
                                                placeholder="Khairul Islam"
                                                prop:value=move || contact_name.get()
                                                on:input=move |ev| contact_name.set(event_target_value(&ev))
                                                class="w-full px-4 py-2.5 rounded-lg bg-slate-800/80 border border-slate-700 text-sm text-white focus:outline-none focus:border-orange-500 transition-colors"
                                            />
                                        </div>
                                        <div class="space-y-1.5">
                                            <label class="text-xs font-medium text-slate-300">"Email Address"</label>
                                            <input
                                                type="email"
                                                placeholder="you@domain.com"
                                                prop:value=move || contact_email.get()
                                                on:input=move |ev| contact_email.set(event_target_value(&ev))
                                                class="w-full px-4 py-2.5 rounded-lg bg-slate-800/80 border border-slate-700 text-sm text-white focus:outline-none focus:border-orange-500 transition-colors"
                                            />
                                        </div>
                                    </div>

                                    <div class="space-y-1.5">
                                        <label class="text-xs font-medium text-slate-300">"Project Details / Message"</label>
                                        <textarea
                                            rows="4"
                                            placeholder="Tell me about your project, timeline, or role..."
                                            prop:value=move || contact_msg.get()
                                            on:input=move |ev| contact_msg.set(event_target_value(&ev))
                                            class="w-full px-4 py-2.5 rounded-lg bg-slate-800/80 border border-slate-700 text-sm text-white focus:outline-none focus:border-orange-500 transition-colors"
                                        ></textarea>
                                    </div>

                                    <button
                                        on:click=move |_| {
                                            if !contact_name.get().is_empty() && !contact_email.get().is_empty() {
                                                contact_submitted.set(true);
                                            }
                                        }
                                        class="w-full py-3.5 rounded-xl bg-gradient-to-r from-orange-600 to-amber-600 hover:from-orange-500 hover:to-amber-500 text-white font-semibold transition-all shadow-lg shadow-orange-600/30 active:scale-[0.99]"
                                    >
                                        "Send Message"
                                    </button>
                                </div>
                            }.into_any()
                        }}
                    </div>
                </div>
            </section>

            // Footer
            <footer class="py-10 px-6 border-t border-slate-800 text-center text-xs text-slate-400 space-y-2">
                <div class="flex items-center justify-center gap-2">
                    <span>"Crafted with"</span>
                    <span class="text-orange-500 font-bold">"Rust 🦀"</span>
                    <span>"+"</span>
                    <span class="text-amber-400 font-bold">"Leptos"</span>
                    <span>"+"</span>
                    <span class="text-cyan-400 font-bold">"WebAssembly"</span>
                </div>
                <p>"© 2026 Khairul. All rights reserved. Zero JavaScript Framework Overhead."</p>
            </footer>
        </div>
    }
}
