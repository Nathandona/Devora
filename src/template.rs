//! Template engine for C++ project generation

use serde::Serialize;
use tera::{Tera, Context};
use crate::config::DevoraConfig;
use crate::result::{Result, DevoraError};

pub struct TemplateEngine {
    tera: Tera,
}

impl TemplateEngine {
    pub fn new() -> Result<Self> {
        let mut tera = match Tera::new("templates/**/*") {
            Ok(tera) => tera,
            Err(_) => {
                // Create empty Tera instance if no templates directory exists yet
                Tera::default()
            }
        };

        // Add built-in templates
        Self::add_builtin_templates(&mut tera)?;

        Ok(Self { tera })
    }

    fn add_builtin_templates(tera: &mut Tera) -> Result<()> {
        // Meson build template
        tera.add_raw_template(
            "meson.build",
            r#"project('{{ project.name }}', 'cpp',
  version : '{{ project.version }}',
  default_options : ['warning_level=3',
                     'cpp_std=c++{{ project.cpp_standard }}'])

# Dependencies
{% for dep in build.dependencies %}
{{ dep.name }}_dep = dependency('{{ dep.name }}')
{% endfor %}

# Executable
executable('{{ project.name }}',
  'src/main.cpp',
  install : true,
  {% if build.dependencies %}
  dependencies : [{% for dep in build.dependencies %}{{ dep.name }}_dep{% if not loop.last %}, {% endif %}{% endfor %}],
  {% endif %}
)

# Tests
{% if test.framework != "none" %}
{{ test.framework }}_dep = dependency('{{ test.framework }}', fallback : ['{{ test.framework }}', 'fallback'])

test('{{ test.framework }}_test',
  executable('test_{{ project.name }}',
    'tests/test_main.cpp',
    dependencies : [{{ test.framework }}_dep]
  )
)
{% endif %}"#,
        ).map_err(|e| DevoraError::template(format!("Failed to add meson.build template: {}", e)))?;

        // Main C++ file template
        tera.add_raw_template(
            "main.cpp",
            r#"#include <iostream>

int main() {
    std::cout << "Hello from {{ project.name }}!" << std::endl;
    return 0;
}"#,
        ).map_err(|e| DevoraError::template(format!("Failed to add main.cpp template: {}", e)))?;

        // Test file template
        tera.add_raw_template(
            "test_main.cpp",
            r#"{% if test.framework == "catch2" %}
#include <catch2/catch_all.hpp>

TEST_CASE("Basic test", "[core]") {
    REQUIRE(1 + 1 == 2);
}

int main(int argc, char* argv[]) {
    return Catch::Session().run(argc, argv);
}
{% elif test.framework == "gtest" %}
#include <gtest/gtest.h>

TEST(BasicTest, Addition) {
    EXPECT_EQ(1 + 1, 2);
}

int main(int argc, char **argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
{% elif test.framework == "doctest" %}
#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest/doctest.h>

TEST_CASE("Basic test") {
    CHECK(1 + 1 == 2);
}
{% endif %}"#,
        ).map_err(|e| DevoraError::template(format!("Failed to add test_main.cpp template: {}", e)))?;

        // Devora config template
        tera.add_raw_template(
            "devora.toml",
            r#"[project]
name = "{{ project.name }}"
version = "{{ project.version }}"
cpp_standard = "{{ project.cpp_standard }}"
{% if project.description -%}
description = "{{ project.description }}"
{% endif -%}
{% if project.authors -%}
authors = [{% for author in project.authors %}"{{ author }}"{% if not loop.last %}, {% endif %}{% endfor %}]
{% endif -%}
{% if project.license -%}
license = "{{ project.license }}"
{% endif -%}

[build]
build_system = "{{ build.build_system }}"
build_dir = "{{ build.build_dir }}"
target_dir = "{{ build.target_dir }}"
{% if build.package_manager %}package_manager = "{{ build.package_manager }}"{% endif %}
dependencies = []

[dev]
port = {{ dev.port }}
auto_reload = {{ dev.auto_reload }}
open_browser = {{ dev.open_browser }}
exclude_patterns = ["build/**", "*.o", "*.so", "*.dll"]

[test]
framework = "{{ test.framework }}"
test_dir = "{{ test.test_dir }}"
test_pattern = "{{ test.test_pattern }}"
coverage = {{ test.coverage }}

[lint]
enabled = {{ lint.enabled }}
tool = "{{ lint.tool }}"
fix_on_save = {{ lint.fix_on_save }}
"#,
        ).map_err(|e| DevoraError::template(format!("Failed to add devora.toml template: {}", e)))?;

        // README template
        tera.add_raw_template(
            "README.md",
            r#"# {{ project.name }}

{% if project.description %}{{ project.description }}{% endif %}

## Build

### Prerequisites

- Meson (>= 0.64.0)
- Ninja
- C++ compiler supporting C++{{ project.cpp_standard }}

### Building

```bash
# Setup build directory
meson setup {{ build.build_dir }}

# Build the project
meson compile -C {{ build.build_dir }}

# Run the application
./{{ build.build_dir }}/{{ project.name }}
```

### Testing

{% if test.framework != "none" %}
```bash
# Run tests
meson test -C {{ build.build_dir }}
```
{% endif %}

## Development with Devora

If you have Devora installed:

```bash
# Start development server with live reload
devora dev

# Run tests
devora test

# Lint code
devora lint

# Build for release
devora build --release
```

## License

{% if project.license %}{{ project.license }}{% else %}MIT{% endif %}"#,
        ).map_err(|e| DevoraError::template(format!("Failed to add README.md template: {}", e)))?;

        Ok(())
    }

    pub fn render(&self, template_name: &str, context: &Context) -> Result<String> {
        self.tera
            .render(template_name, context)
            .map_err(|e| DevoraError::template(format!("Failed to render template {}: {}", template_name, e)))
    }

    pub fn create_context(config: &DevoraConfig) -> Context {
        let mut context = Context::new();
        context.insert("project", &config.project);
        context.insert("build", &config.build);
        context.insert("dev", &config.dev);
        context.insert("test", &config.test);
        context.insert("lint", &config.lint);
        context
    }
}

#[derive(Serialize)]
pub struct TemplateVariables {
    pub project_name: String,
    pub cpp_standard: String,
    pub test_framework: String,
    pub package_manager: String,
}

impl TemplateVariables {
    pub fn from_config(config: &DevoraConfig) -> Self {
        Self {
            project_name: config.project.name.clone(),
            cpp_standard: config.project.cpp_standard.clone(),
            test_framework: config.test.framework.clone(),
            package_manager: config.build.package_manager.clone().unwrap_or_default(),
        }
    }
}