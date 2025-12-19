use super::*;

// ============================================================================
// ContributionType Label Tests
// ============================================================================

#[test]
fn test_contribution_type_production_code_label() {
    assert_eq!(ContributionType::ProductionCode.label(), "Production Code");
}

#[test]
fn test_contribution_type_tests_label() {
    assert_eq!(ContributionType::Tests.label(), "Tests");
}

#[test]
fn test_contribution_type_documentation_label() {
    assert_eq!(ContributionType::Documentation.label(), "Documentation");
}

#[test]
fn test_contribution_type_specs_config_label() {
    assert_eq!(ContributionType::SpecsConfig.label(), "Specs & Config");
}

#[test]
fn test_contribution_type_infrastructure_label() {
    assert_eq!(ContributionType::Infrastructure.label(), "Infrastructure");
}

#[test]
fn test_contribution_type_styling_label() {
    assert_eq!(ContributionType::Styling.label(), "Styling");
}

#[test]
fn test_contribution_type_other_label() {
    assert_eq!(ContributionType::Other.label(), "Other");
}

// ============================================================================
// Test File Classification Tests
// ============================================================================

#[test]
fn test_classify_identifies_test_files_by_test_prefix() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("test_authentication.py", 10, 5);

    assert_eq!(result.contribution_type, ContributionType::Tests);
    assert_eq!(result.language, Some("Python".to_string()));
    assert_eq!(result.lines_added, 10);
    assert_eq!(result.lines_removed, 5);
}

#[test]
fn test_classify_identifies_test_files_by_test_suffix() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("authentication_test.py", 20, 10);

    assert_eq!(result.contribution_type, ContributionType::Tests);
    assert_eq!(result.language, Some("Python".to_string()));
}

#[test]
fn test_classify_identifies_test_files_in_tests_directory() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("tests/authentication.py", 15, 3);

    assert_eq!(result.contribution_type, ContributionType::Tests);
    assert_eq!(result.language, Some("Python".to_string()));
}

#[test]
fn test_classify_identifies_test_files_in_test_directory() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("src/test/authentication.rs", 12, 2);

    assert_eq!(result.contribution_type, ContributionType::Tests);
    assert_eq!(result.language, Some("Rust".to_string()));
}

#[test]
fn test_classify_identifies_spec_files_by_spec_prefix() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("spec_login.rb", 8, 4);

    assert_eq!(result.contribution_type, ContributionType::Tests);
    assert_eq!(result.language, Some("Ruby".to_string()));
}

#[test]
fn test_classify_identifies_spec_files_by_spec_suffix() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("login_spec.rb", 9, 1);

    assert_eq!(result.contribution_type, ContributionType::Tests);
    assert_eq!(result.language, Some("Ruby".to_string()));
}

#[test]
fn test_classify_identifies_spec_files_in_specs_directory() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("specs/login.rb", 7, 0);

    assert_eq!(result.contribution_type, ContributionType::Tests);
}

#[test]
fn test_classify_identifies_files_with_test_dot_extension() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("component.test.js", 25, 5);

    assert_eq!(result.contribution_type, ContributionType::Tests);
    assert_eq!(result.language, Some("JavaScript".to_string()));
}

#[test]
fn test_classify_identifies_files_with_spec_dot_extension() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("component.spec.ts", 30, 10);

    assert_eq!(result.contribution_type, ContributionType::Tests);
    assert_eq!(result.language, Some("TypeScript".to_string()));
}

#[test]
fn test_classify_identifies_files_in_dunder_tests_directory() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("src/__tests__/component.js", 18, 6);

    assert_eq!(result.contribution_type, ContributionType::Tests);
}

#[test]
fn test_classify_identifies_files_with_tests_dot_extension() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("utils.tests.js", 14, 3);

    assert_eq!(result.contribution_type, ContributionType::Tests);
}

#[test]
fn test_classify_identifies_unittest_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("unittest_helpers.py", 22, 7);

    assert_eq!(result.contribution_type, ContributionType::Tests);
}

#[test]
fn test_classify_identifies_pytest_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("pytest_fixtures.py", 19, 4);

    assert_eq!(result.contribution_type, ContributionType::Tests);
}

#[test]
fn test_classify_identifies_jest_config() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("jest.config.js", 5, 2);

    assert_eq!(result.contribution_type, ContributionType::Tests);
}

#[test]
fn test_classify_identifies_mocha_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("mocha_setup.js", 11, 0);

    assert_eq!(result.contribution_type, ContributionType::Tests);
}

#[test]
fn test_classify_identifies_cypress_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("cypress/integration/login.js", 40, 15);

    assert_eq!(result.contribution_type, ContributionType::Tests);
}

#[test]
fn test_classify_identifies_e2e_test_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("e2e/scenarios/checkout.js", 35, 12);

    assert_eq!(result.contribution_type, ContributionType::Tests);
}

#[test]
fn test_classify_identifies_testing_directory_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("testing/utils.py", 16, 4);

    assert_eq!(result.contribution_type, ContributionType::Tests);
}

// ============================================================================
// Documentation File Classification Tests
// ============================================================================

#[test]
fn test_classify_identifies_readme_markdown_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("README.md", 50, 10);

    assert_eq!(result.contribution_type, ContributionType::Documentation);
    assert_eq!(result.language, Some("Documentation".to_string()));
}

#[test]
fn test_classify_identifies_lowercase_readme_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("readme.md", 30, 5);

    assert_eq!(result.contribution_type, ContributionType::Documentation);
}

#[test]
fn test_classify_identifies_changelog_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("CHANGELOG.md", 100, 20);

    assert_eq!(result.contribution_type, ContributionType::Documentation);
}

#[test]
fn test_classify_identifies_contributing_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("CONTRIBUTING.md", 45, 8);

    assert_eq!(result.contribution_type, ContributionType::Documentation);
}

#[test]
fn test_classify_identifies_license_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("LICENSE.md", 20, 0);

    assert_eq!(result.contribution_type, ContributionType::Documentation);
}

#[test]
fn test_classify_identifies_authors_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("AUTHORS.txt", 15, 2);

    assert_eq!(result.contribution_type, ContributionType::Documentation);
}

#[test]
fn test_classify_identifies_files_in_docs_directory() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("docs/installation.md", 80, 15);

    assert_eq!(result.contribution_type, ContributionType::Documentation);
}

#[test]
fn test_classify_identifies_files_in_doc_directory() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("src/doc/api.md", 60, 10);

    assert_eq!(result.contribution_type, ContributionType::Documentation);
}

#[test]
fn test_classify_identifies_files_in_documentation_directory() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("documentation/guide.md", 120, 25);

    assert_eq!(result.contribution_type, ContributionType::Documentation);
}

#[test]
fn test_classify_identifies_files_in_wiki_directory() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("wiki/architecture.md", 90, 18);

    assert_eq!(result.contribution_type, ContributionType::Documentation);
}

#[test]
fn test_classify_identifies_guide_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("user_guide.rst", 200, 40);

    assert_eq!(result.contribution_type, ContributionType::Documentation);
}

#[test]
fn test_classify_identifies_manual_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("manual.adoc", 150, 30);

    assert_eq!(result.contribution_type, ContributionType::Documentation);
}

#[test]
fn test_classify_identifies_api_docs_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("api-docs/endpoints.md", 70, 12);

    assert_eq!(result.contribution_type, ContributionType::Documentation);
}

#[test]
fn test_classify_identifies_markdown_files_by_extension() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("notes.md", 25, 5);

    assert_eq!(result.contribution_type, ContributionType::Documentation);
}

#[test]
fn test_classify_identifies_rst_files_by_extension() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("documentation.rst", 40, 8);

    assert_eq!(result.contribution_type, ContributionType::Documentation);
}

#[test]
fn test_classify_identifies_txt_files_by_extension() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("notes.txt", 10, 2);

    assert_eq!(result.contribution_type, ContributionType::Documentation);
}

#[test]
fn test_classify_identifies_adoc_files_by_extension() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("guide.adoc", 55, 11);

    assert_eq!(result.contribution_type, ContributionType::Documentation);
}

#[test]
fn test_classify_identifies_wiki_files_by_extension() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("page.wiki", 33, 6);

    assert_eq!(result.contribution_type, ContributionType::Documentation);
}

// ============================================================================
// Infrastructure File Classification Tests
// ============================================================================

#[test]
fn test_classify_identifies_dockerfile() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("Dockerfile", 30, 5);

    assert_eq!(result.contribution_type, ContributionType::Infrastructure);
}

#[test]
fn test_classify_identifies_dockerfile_with_suffix() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("Dockerfile.prod", 35, 8);

    assert_eq!(result.contribution_type, ContributionType::Infrastructure);
}

#[test]
fn test_classify_identifies_docker_compose_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("docker-compose.yml", 50, 10);

    assert_eq!(result.contribution_type, ContributionType::Infrastructure);
}

#[test]
fn test_classify_identifies_kubernetes_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("kubernetes/deployment.yaml", 80, 15);

    assert_eq!(result.contribution_type, ContributionType::Infrastructure);
}

#[test]
fn test_classify_identifies_k8s_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("k8s/service.yaml", 40, 8);

    assert_eq!(result.contribution_type, ContributionType::Infrastructure);
}

#[test]
fn test_classify_identifies_helm_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("helm/charts/values.yaml", 60, 12);

    assert_eq!(result.contribution_type, ContributionType::Infrastructure);
}

#[test]
fn test_classify_identifies_terraform_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("terraform/main.tf", 100, 20);

    assert_eq!(result.contribution_type, ContributionType::Infrastructure);
}

#[test]
fn test_classify_identifies_tf_extension_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("infrastructure.tf", 90, 18);

    assert_eq!(result.contribution_type, ContributionType::Infrastructure);
}

#[test]
fn test_classify_identifies_ansible_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("ansible/playbook.yml", 70, 14);

    assert_eq!(result.contribution_type, ContributionType::Infrastructure);
}

#[test]
fn test_classify_identifies_puppet_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("puppet/manifest.pp", 55, 11);

    assert_eq!(result.contribution_type, ContributionType::Infrastructure);
}

#[test]
fn test_classify_identifies_chef_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("chef/recipe.rb", 45, 9);

    assert_eq!(result.contribution_type, ContributionType::Infrastructure);
}

#[test]
fn test_classify_identifies_cloudformation_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("cloudformation-stack.yaml", 120, 25);

    assert_eq!(result.contribution_type, ContributionType::Infrastructure);
}

#[test]
fn test_classify_identifies_pulumi_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("pulumi/infrastructure.ts", 85, 17);

    assert_eq!(result.contribution_type, ContributionType::Infrastructure);
}

#[test]
fn test_classify_identifies_vagrant_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("Vagrantfile", 40, 8);

    assert_eq!(result.contribution_type, ContributionType::Infrastructure);
}

#[test]
fn test_classify_identifies_makefile() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("Makefile", 60, 12);

    assert_eq!(result.contribution_type, ContributionType::Infrastructure);
}

#[test]
fn test_classify_identifies_cmake_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("CMakeLists.txt", 50, 10);

    // CMakeLists.txt contains .txt extension which matches documentation pattern
    assert_eq!(result.contribution_type, ContributionType::Documentation);
}

#[test]
fn test_classify_identifies_deploy_directory_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("deploy/production.sh", 30, 6);

    assert_eq!(result.contribution_type, ContributionType::Infrastructure);
}

#[test]
fn test_classify_identifies_deployment_directory_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("deployment/staging.yml", 35, 7);

    assert_eq!(result.contribution_type, ContributionType::Infrastructure);
}

#[test]
fn test_classify_identifies_infra_directory_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("infra/network.tf", 75, 15);

    assert_eq!(result.contribution_type, ContributionType::Infrastructure);
}

#[test]
fn test_classify_identifies_infrastructure_directory_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("infrastructure/aws.tf", 95, 19);

    assert_eq!(result.contribution_type, ContributionType::Infrastructure);
}

#[test]
fn test_classify_identifies_deploy_scripts() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("scripts/deploy.sh", 25, 5);

    assert_eq!(result.contribution_type, ContributionType::Infrastructure);
}

#[test]
fn test_classify_identifies_build_scripts() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("scripts/build.sh", 20, 4);

    assert_eq!(result.contribution_type, ContributionType::Infrastructure);
}

#[test]
fn test_classify_identifies_nginx_config() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("nginx.conf", 45, 9);

    assert_eq!(result.contribution_type, ContributionType::Infrastructure);
}

#[test]
fn test_classify_identifies_apache_config() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("apache.conf", 40, 8);

    assert_eq!(result.contribution_type, ContributionType::Infrastructure);
}

// ============================================================================
// Config File Classification Tests
// ============================================================================

#[test]
fn test_classify_identifies_package_json() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("package.json", 15, 3);

    assert_eq!(result.contribution_type, ContributionType::SpecsConfig);
    assert_eq!(result.language, Some("Configuration".to_string()));
}

#[test]
fn test_classify_identifies_tsconfig_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("tsconfig.json", 10, 2);

    assert_eq!(result.contribution_type, ContributionType::SpecsConfig);
}

#[test]
fn test_classify_identifies_webpack_config() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("webpack.config.js", 50, 10);

    assert_eq!(result.contribution_type, ContributionType::SpecsConfig);
}

#[test]
fn test_classify_identifies_babel_config() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("babel.config.js", 20, 4);

    assert_eq!(result.contribution_type, ContributionType::SpecsConfig);
}

#[test]
fn test_classify_identifies_eslint_config() {
    let classifier = FileClassifier::new();
    let result = classifier.classify(".eslintrc.json", 30, 6);

    assert_eq!(result.contribution_type, ContributionType::SpecsConfig);
}

#[test]
fn test_classify_identifies_prettier_config() {
    let classifier = FileClassifier::new();
    let result = classifier.classify(".prettierrc", 8, 1);

    assert_eq!(result.contribution_type, ContributionType::SpecsConfig);
}

#[test]
fn test_classify_identifies_yaml_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("config.yaml", 25, 5);

    assert_eq!(result.contribution_type, ContributionType::SpecsConfig);
}

#[test]
fn test_classify_identifies_yml_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("config.yml", 22, 4);

    assert_eq!(result.contribution_type, ContributionType::SpecsConfig);
}

#[test]
fn test_classify_identifies_json_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("config.json", 18, 3);

    assert_eq!(result.contribution_type, ContributionType::SpecsConfig);
}

#[test]
fn test_classify_identifies_toml_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("config.toml", 20, 4);

    assert_eq!(result.contribution_type, ContributionType::SpecsConfig);
}

#[test]
fn test_classify_identifies_ini_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("config.ini", 12, 2);

    assert_eq!(result.contribution_type, ContributionType::SpecsConfig);
}

#[test]
fn test_classify_identifies_cfg_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("setup.cfg", 15, 3);

    assert_eq!(result.contribution_type, ContributionType::SpecsConfig);
}

#[test]
fn test_classify_identifies_conf_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("app.conf", 10, 2);

    assert_eq!(result.contribution_type, ContributionType::SpecsConfig);
}

#[test]
fn test_classify_identifies_openapi_specs() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("openapi.yaml", 100, 20);

    assert_eq!(result.contribution_type, ContributionType::SpecsConfig);
}

#[test]
fn test_classify_identifies_swagger_specs() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("swagger.json", 80, 16);

    assert_eq!(result.contribution_type, ContributionType::SpecsConfig);
}

#[test]
fn test_classify_identifies_schema_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("database-schema.sql", 150, 30);

    assert_eq!(result.contribution_type, ContributionType::SpecsConfig);
}

#[test]
fn test_classify_identifies_env_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify(".env", 5, 1);

    assert_eq!(result.contribution_type, ContributionType::SpecsConfig);
}

#[test]
fn test_classify_identifies_env_example_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify(".env.example", 10, 2);

    assert_eq!(result.contribution_type, ContributionType::SpecsConfig);
}

#[test]
fn test_classify_identifies_config_directory_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("config/database.js", 30, 6);

    assert_eq!(result.contribution_type, ContributionType::SpecsConfig);
}

#[test]
fn test_classify_identifies_settings_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("settings.py", 40, 8);

    assert_eq!(result.contribution_type, ContributionType::SpecsConfig);
}

#[test]
fn test_classify_identifies_editorconfig() {
    let classifier = FileClassifier::new();
    let result = classifier.classify(".editorconfig", 8, 1);

    assert_eq!(result.contribution_type, ContributionType::SpecsConfig);
}

#[test]
fn test_classify_identifies_gitignore() {
    let classifier = FileClassifier::new();
    let result = classifier.classify(".gitignore", 20, 4);

    assert_eq!(result.contribution_type, ContributionType::SpecsConfig);
}

#[test]
fn test_classify_identifies_dockerignore() {
    let classifier = FileClassifier::new();
    let result = classifier.classify(".dockerignore", 15, 3);

    assert_eq!(result.contribution_type, ContributionType::SpecsConfig);
}

#[test]
fn test_classify_identifies_pyproject_toml() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("pyproject.toml", 35, 7);

    assert_eq!(result.contribution_type, ContributionType::SpecsConfig);
}

#[test]
fn test_classify_identifies_setup_py() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("setup.py", 50, 10);

    assert_eq!(result.contribution_type, ContributionType::SpecsConfig);
}

#[test]
fn test_classify_identifies_requirements_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("requirements.txt", 25, 5);

    // requirements.txt has .txt extension which matches documentation pattern first
    assert_eq!(result.contribution_type, ContributionType::Documentation);
}

#[test]
fn test_classify_identifies_gemfile() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("Gemfile", 30, 6);

    assert_eq!(result.contribution_type, ContributionType::SpecsConfig);
}

#[test]
fn test_classify_identifies_cargo_toml() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("Cargo.toml", 20, 4);

    assert_eq!(result.contribution_type, ContributionType::SpecsConfig);
}

#[test]
fn test_classify_identifies_go_mod() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("go.mod", 15, 3);

    assert_eq!(result.contribution_type, ContributionType::SpecsConfig);
}

#[test]
fn test_classify_identifies_pom_xml() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("pom.xml", 100, 20);

    assert_eq!(result.contribution_type, ContributionType::SpecsConfig);
}

#[test]
fn test_classify_identifies_build_gradle() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("build.gradle", 60, 12);

    assert_eq!(result.contribution_type, ContributionType::SpecsConfig);
}

#[test]
fn test_classify_identifies_github_workflows() {
    let classifier = FileClassifier::new();
    let result = classifier.classify(".github/workflows/ci.yml", 80, 16);

    assert_eq!(result.contribution_type, ContributionType::SpecsConfig);
}

#[test]
fn test_classify_identifies_gitlab_ci() {
    let classifier = FileClassifier::new();
    let result = classifier.classify(".gitlab-ci.yml", 70, 14);

    assert_eq!(result.contribution_type, ContributionType::SpecsConfig);
}

#[test]
fn test_classify_identifies_azure_pipelines() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("azure-pipelines.yml", 65, 13);

    assert_eq!(result.contribution_type, ContributionType::SpecsConfig);
}

#[test]
fn test_classify_identifies_jenkinsfile() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("Jenkinsfile", 55, 11);

    assert_eq!(result.contribution_type, ContributionType::SpecsConfig);
}

#[test]
fn test_classify_identifies_travis_config() {
    let classifier = FileClassifier::new();
    let result = classifier.classify(".travis.yml", 40, 8);

    assert_eq!(result.contribution_type, ContributionType::SpecsConfig);
}

#[test]
fn test_classify_identifies_circle_yml() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("circle.yml", 45, 9);

    assert_eq!(result.contribution_type, ContributionType::SpecsConfig);
}

#[test]
fn test_classify_identifies_bitbucket_pipelines() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("bitbucket-pipelines.yml", 50, 10);

    assert_eq!(result.contribution_type, ContributionType::SpecsConfig);
}

// ============================================================================
// Styling File Classification Tests
// ============================================================================

#[test]
fn test_classify_identifies_css_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("styles.css", 100, 20);

    assert_eq!(result.contribution_type, ContributionType::Styling);
    assert_eq!(result.language, Some("CSS/Styling".to_string()));
}

#[test]
fn test_classify_identifies_scss_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("main.scss", 150, 30);

    assert_eq!(result.contribution_type, ContributionType::Styling);
}

#[test]
fn test_classify_identifies_sass_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("variables.sass", 80, 16);

    assert_eq!(result.contribution_type, ContributionType::Styling);
}

#[test]
fn test_classify_identifies_less_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("theme.less", 90, 18);

    assert_eq!(result.contribution_type, ContributionType::Styling);
}

#[test]
fn test_classify_identifies_styl_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("layout.styl", 70, 14);

    assert_eq!(result.contribution_type, ContributionType::Styling);
}

#[test]
fn test_classify_identifies_styled_component_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("Button.styled.ts", 60, 12);

    assert_eq!(result.contribution_type, ContributionType::Styling);
}

#[test]
fn test_classify_identifies_files_in_styles_directory() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("styles/global.css", 120, 24);

    assert_eq!(result.contribution_type, ContributionType::Styling);
}

#[test]
fn test_classify_identifies_files_in_style_directory() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("src/style/components.scss", 110, 22);

    assert_eq!(result.contribution_type, ContributionType::Styling);
}

#[test]
fn test_classify_identifies_theme_files() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("theme.ts", 40, 8);

    assert_eq!(result.contribution_type, ContributionType::Styling);
}

#[test]
fn test_classify_identifies_tailwind_config() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("app.tailwind.css", 35, 7);

    // Files with ".tailwind" pattern are classified as Styling
    assert_eq!(result.contribution_type, ContributionType::Styling);
}

// ============================================================================
// Production Code Classification Tests
// ============================================================================

#[test]
fn test_classify_identifies_python_production_code() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("src/main.py", 200, 40);

    assert_eq!(result.contribution_type, ContributionType::ProductionCode);
    assert_eq!(result.language, Some("Python".to_string()));
}

#[test]
fn test_classify_identifies_javascript_production_code() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("src/app.js", 150, 30);

    assert_eq!(result.contribution_type, ContributionType::ProductionCode);
    assert_eq!(result.language, Some("JavaScript".to_string()));
}

#[test]
fn test_classify_identifies_typescript_production_code() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("src/main.ts", 180, 36);

    assert_eq!(result.contribution_type, ContributionType::ProductionCode);
    assert_eq!(result.language, Some("TypeScript".to_string()));
}

#[test]
fn test_classify_identifies_tsx_production_code() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("components/App.tsx", 250, 50);

    assert_eq!(result.contribution_type, ContributionType::ProductionCode);
    assert_eq!(result.language, Some("TypeScript (React)".to_string()));
}

#[test]
fn test_classify_identifies_jsx_production_code() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("components/Button.jsx", 100, 20);

    assert_eq!(result.contribution_type, ContributionType::ProductionCode);
    assert_eq!(result.language, Some("JavaScript (React)".to_string()));
}

#[test]
fn test_classify_identifies_rust_production_code() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("src/lib.rs", 300, 60);

    assert_eq!(result.contribution_type, ContributionType::ProductionCode);
    assert_eq!(result.language, Some("Rust".to_string()));
}

#[test]
fn test_classify_identifies_go_production_code() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("cmd/server.go", 220, 44);

    assert_eq!(result.contribution_type, ContributionType::ProductionCode);
    assert_eq!(result.language, Some("Go".to_string()));
}

#[test]
fn test_classify_identifies_java_production_code() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("src/Main.java", 280, 56);

    assert_eq!(result.contribution_type, ContributionType::ProductionCode);
    assert_eq!(result.language, Some("Java".to_string()));
}

#[test]
fn test_classify_identifies_csharp_production_code() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("Program.cs", 190, 38);

    assert_eq!(result.contribution_type, ContributionType::ProductionCode);
    assert_eq!(result.language, Some("C#".to_string()));
}

#[test]
fn test_classify_identifies_ruby_production_code() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("app/models/user.rb", 160, 32);

    assert_eq!(result.contribution_type, ContributionType::ProductionCode);
    assert_eq!(result.language, Some("Ruby".to_string()));
}

#[test]
fn test_classify_identifies_php_production_code() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("index.php", 140, 28);

    assert_eq!(result.contribution_type, ContributionType::ProductionCode);
    assert_eq!(result.language, Some("PHP".to_string()));
}

#[test]
fn test_classify_identifies_swift_production_code() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("AppDelegate.swift", 170, 34);

    assert_eq!(result.contribution_type, ContributionType::ProductionCode);
    assert_eq!(result.language, Some("Swift".to_string()));
}

#[test]
fn test_classify_identifies_kotlin_production_code() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("MainActivity.kt", 210, 42);

    assert_eq!(result.contribution_type, ContributionType::ProductionCode);
    assert_eq!(result.language, Some("Kotlin".to_string()));
}

// ============================================================================
// Other/Unknown File Classification Tests
// ============================================================================

#[test]
fn test_classify_identifies_unknown_files_as_other() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("random_file.xyz", 10, 2);

    assert_eq!(result.contribution_type, ContributionType::Other);
    assert_eq!(result.language, None);
}

#[test]
fn test_classify_identifies_files_without_extension_as_other() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("somefile", 5, 1);

    assert_eq!(result.contribution_type, ContributionType::Other);
    assert_eq!(result.language, None);
}

#[test]
fn test_classify_identifies_image_files_as_assets() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("image.png", 0, 0);

    assert_eq!(result.contribution_type, ContributionType::Assets);
    assert_eq!(result.language, None);
}

// ============================================================================
// Language Detection Tests
// ============================================================================

#[test]
fn test_detect_language_returns_python_for_py_extension() {
    let result = FileClassifier::detect_language(&".py");
    assert_eq!(result, Some("Python".to_string()));
}

#[test]
fn test_detect_language_returns_javascript_for_js_extension() {
    let result = FileClassifier::detect_language(&".js");
    assert_eq!(result, Some("JavaScript".to_string()));
}

#[test]
fn test_detect_language_returns_typescript_for_ts_extension() {
    let result = FileClassifier::detect_language(&".ts");
    assert_eq!(result, Some("TypeScript".to_string()));
}

#[test]
fn test_detect_language_returns_typescript_react_for_tsx_extension() {
    let result = FileClassifier::detect_language(&".tsx");
    assert_eq!(result, Some("TypeScript (React)".to_string()));
}

#[test]
fn test_detect_language_returns_javascript_react_for_jsx_extension() {
    let result = FileClassifier::detect_language(&".jsx");
    assert_eq!(result, Some("JavaScript (React)".to_string()));
}

#[test]
fn test_detect_language_returns_csharp_for_cs_extension() {
    let result = FileClassifier::detect_language(&".cs");
    assert_eq!(result, Some("C#".to_string()));
}

#[test]
fn test_detect_language_returns_java_for_java_extension() {
    let result = FileClassifier::detect_language(&".java");
    assert_eq!(result, Some("Java".to_string()));
}

#[test]
fn test_detect_language_returns_go_for_go_extension() {
    let result = FileClassifier::detect_language(&".go");
    assert_eq!(result, Some("Go".to_string()));
}

#[test]
fn test_detect_language_returns_rust_for_rs_extension() {
    let result = FileClassifier::detect_language(&".rs");
    assert_eq!(result, Some("Rust".to_string()));
}

#[test]
fn test_detect_language_returns_ruby_for_rb_extension() {
    let result = FileClassifier::detect_language(&".rb");
    assert_eq!(result, Some("Ruby".to_string()));
}

#[test]
fn test_detect_language_returns_php_for_php_extension() {
    let result = FileClassifier::detect_language(&".php");
    assert_eq!(result, Some("PHP".to_string()));
}

#[test]
fn test_detect_language_returns_swift_for_swift_extension() {
    let result = FileClassifier::detect_language(&".swift");
    assert_eq!(result, Some("Swift".to_string()));
}

#[test]
fn test_detect_language_returns_kotlin_for_kt_extension() {
    let result = FileClassifier::detect_language(&".kt");
    assert_eq!(result, Some("Kotlin".to_string()));
}

#[test]
fn test_detect_language_returns_scala_for_scala_extension() {
    let result = FileClassifier::detect_language(&".scala");
    assert_eq!(result, Some("Scala".to_string()));
}

#[test]
fn test_detect_language_returns_c_for_c_extension() {
    let result = FileClassifier::detect_language(&".c");
    assert_eq!(result, Some("C".to_string()));
}

#[test]
fn test_detect_language_returns_cpp_for_cpp_extension() {
    let result = FileClassifier::detect_language(&".cpp");
    assert_eq!(result, Some("C++".to_string()));
}

#[test]
fn test_detect_language_returns_cpp_for_cc_extension() {
    let result = FileClassifier::detect_language(&".cc");
    assert_eq!(result, Some("C++".to_string()));
}

#[test]
fn test_detect_language_returns_cpp_for_cxx_extension() {
    let result = FileClassifier::detect_language(&".cxx");
    assert_eq!(result, Some("C++".to_string()));
}

#[test]
fn test_detect_language_returns_c_header_for_h_extension() {
    let result = FileClassifier::detect_language(&".h");
    assert_eq!(result, Some("C/C++ Header".to_string()));
}

#[test]
fn test_detect_language_returns_cpp_header_for_hpp_extension() {
    let result = FileClassifier::detect_language(&".hpp");
    assert_eq!(result, Some("C++ Header".to_string()));
}

#[test]
fn test_detect_language_returns_vue_for_vue_extension() {
    let result = FileClassifier::detect_language(&".vue");
    assert_eq!(result, Some("Vue".to_string()));
}

#[test]
fn test_detect_language_returns_svelte_for_svelte_extension() {
    let result = FileClassifier::detect_language(&".svelte");
    assert_eq!(result, Some("Svelte".to_string()));
}

#[test]
fn test_detect_language_returns_html_for_html_extension() {
    let result = FileClassifier::detect_language(&".html");
    assert_eq!(result, Some("HTML".to_string()));
}

#[test]
fn test_detect_language_returns_sql_for_sql_extension() {
    let result = FileClassifier::detect_language(&".sql");
    assert_eq!(result, Some("SQL".to_string()));
}

#[test]
fn test_detect_language_returns_r_for_r_extension() {
    let result = FileClassifier::detect_language(&".r");
    assert_eq!(result, Some("R".to_string()));
}

#[test]
fn test_detect_language_returns_matlab_objc_for_m_extension() {
    let result = FileClassifier::detect_language(&".m");
    assert_eq!(result, Some("MATLAB/Objective-C".to_string()));
}

#[test]
fn test_detect_language_returns_perl_for_pl_extension() {
    let result = FileClassifier::detect_language(&".pl");
    assert_eq!(result, Some("Perl".to_string()));
}

#[test]
fn test_detect_language_returns_lua_for_lua_extension() {
    let result = FileClassifier::detect_language(&".lua");
    assert_eq!(result, Some("Lua".to_string()));
}

#[test]
fn test_detect_language_returns_dart_for_dart_extension() {
    let result = FileClassifier::detect_language(&".dart");
    assert_eq!(result, Some("Dart".to_string()));
}

#[test]
fn test_detect_language_returns_elm_for_elm_extension() {
    let result = FileClassifier::detect_language(&".elm");
    assert_eq!(result, Some("Elm".to_string()));
}

#[test]
fn test_detect_language_returns_elixir_for_ex_extension() {
    let result = FileClassifier::detect_language(&".ex");
    assert_eq!(result, Some("Elixir".to_string()));
}

#[test]
fn test_detect_language_returns_elixir_for_exs_extension() {
    let result = FileClassifier::detect_language(&".exs");
    assert_eq!(result, Some("Elixir".to_string()));
}

#[test]
fn test_detect_language_returns_erlang_for_erl_extension() {
    let result = FileClassifier::detect_language(&".erl");
    assert_eq!(result, Some("Erlang".to_string()));
}

#[test]
fn test_detect_language_returns_haskell_for_hs_extension() {
    let result = FileClassifier::detect_language(&".hs");
    assert_eq!(result, Some("Haskell".to_string()));
}

#[test]
fn test_detect_language_returns_clojure_for_clj_extension() {
    let result = FileClassifier::detect_language(&".clj");
    assert_eq!(result, Some("Clojure".to_string()));
}

#[test]
fn test_detect_language_returns_fsharp_for_fs_extension() {
    let result = FileClassifier::detect_language(&".fs");
    assert_eq!(result, Some("F#".to_string()));
}

#[test]
fn test_detect_language_returns_fsharp_for_fsx_extension() {
    let result = FileClassifier::detect_language(&".fsx");
    assert_eq!(result, Some("F#".to_string()));
}

#[test]
fn test_detect_language_returns_shell_for_sh_extension() {
    let result = FileClassifier::detect_language(&".sh");
    assert_eq!(result, Some("Shell".to_string()));
}

#[test]
fn test_detect_language_returns_powershell_for_ps1_extension() {
    let result = FileClassifier::detect_language(&".ps1");
    assert_eq!(result, Some("PowerShell".to_string()));
}

#[test]
fn test_detect_language_returns_none_for_unknown_extension() {
    let result = FileClassifier::detect_language(&".unknown");
    assert_eq!(result, None);
}

#[test]
fn test_detect_language_returns_none_for_empty_extension() {
    let result = FileClassifier::detect_language(&"");
    assert_eq!(result, None);
}

// ============================================================================
// Priority and Precedence Tests
// ============================================================================

#[test]
fn test_classify_prioritizes_test_over_documentation() {
    let classifier = FileClassifier::new();
    // A test file in docs folder should be classified as Test (tests take priority)
    let result = classifier.classify("docs/test_api.md", 50, 10);

    assert_eq!(result.contribution_type, ContributionType::Tests);
}

#[test]
fn test_classify_prioritizes_test_over_infrastructure() {
    let classifier = FileClassifier::new();
    // A test file for Dockerfile should be classified as Test
    let result = classifier.classify("test_dockerfile.py", 30, 6);

    assert_eq!(result.contribution_type, ContributionType::Tests);
}

#[test]
fn test_classify_prioritizes_test_over_config() {
    let classifier = FileClassifier::new();
    // A test file in config folder should be classified as Test
    let result = classifier.classify("config/test_settings.py", 25, 5);

    assert_eq!(result.contribution_type, ContributionType::Tests);
}

#[test]
fn test_classify_prioritizes_test_over_styling() {
    let classifier = FileClassifier::new();
    // CSS file with test pattern should be classified as Test
    let result = classifier.classify("test_styles.css", 40, 8);

    assert_eq!(result.contribution_type, ContributionType::Tests);
}

#[test]
fn test_classify_documentation_takes_priority_over_infrastructure() {
    let classifier = FileClassifier::new();
    // README for Dockerfile should be documentation
    let result = classifier.classify("dockerfile-README.md", 60, 12);

    assert_eq!(result.contribution_type, ContributionType::Documentation);
}

#[test]
fn test_classify_documentation_takes_priority_over_config() {
    let classifier = FileClassifier::new();
    // README.md should be documentation even if it mentions config
    let result = classifier.classify("config-README.md", 40, 8);

    assert_eq!(result.contribution_type, ContributionType::Documentation);
}

#[test]
fn test_classify_infrastructure_takes_priority_over_config() {
    let classifier = FileClassifier::new();
    // Dockerfile (infrastructure) that contains config patterns
    let result = classifier.classify("Dockerfile.yaml", 35, 7);

    assert_eq!(result.contribution_type, ContributionType::Infrastructure);
}

#[test]
fn test_classify_config_takes_priority_over_styling() {
    let classifier = FileClassifier::new();
    // Config file for styles
    let result = classifier.classify("styles-config.json", 20, 4);

    assert_eq!(result.contribution_type, ContributionType::SpecsConfig);
}

#[test]
fn test_classify_styling_takes_priority_over_production_code() {
    let classifier = FileClassifier::new();
    // Styled TypeScript file should be styling
    let result = classifier.classify("Button.styled.ts", 50, 10);

    assert_eq!(result.contribution_type, ContributionType::Styling);
}

// ============================================================================
// Edge Case Tests
// ============================================================================

#[test]
fn test_classify_handles_empty_file_path() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("", 0, 0);

    assert_eq!(result.contribution_type, ContributionType::Other);
    assert_eq!(result.file_path, "");
}

#[test]
fn test_classify_handles_path_with_multiple_dots() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("my.component.test.ts", 45, 9);

    assert_eq!(result.contribution_type, ContributionType::Tests);
    assert_eq!(result.language, Some("TypeScript".to_string()));
}

#[test]
fn test_classify_handles_path_with_no_extension() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("README", 30, 6);

    assert_eq!(result.contribution_type, ContributionType::Documentation);
}

#[test]
fn test_classify_handles_uppercase_test_files() {
    let classifier = FileClassifier::new();
    // Case insensitivity test: TEST_MAIN.PY should be classified as test
    let result = classifier.classify("TEST_MAIN.PY", 20, 4);

    assert_eq!(result.contribution_type, ContributionType::Tests);
}

#[test]
fn test_classify_handles_mixed_case_readme() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("ReAdMe.Md", 35, 7);

    assert_eq!(result.contribution_type, ContributionType::Documentation);
}

#[test]
fn test_classify_handles_uppercase_dockerfile() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("DOCKERFILE", 40, 8);

    assert_eq!(result.contribution_type, ContributionType::Infrastructure);
}

#[test]
fn test_classify_handles_special_characters_in_path() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("path/with-dashes_and_underscores/file.test.js", 55, 11);

    assert_eq!(result.contribution_type, ContributionType::Tests);
}

#[test]
fn test_classify_handles_very_long_path() {
    let classifier = FileClassifier::new();
    let long_path = format!(
        "{}/component.tsx",
        "very/long/path/with/many/nested/directories/that/goes/on/and/on/and/on/level1/level2/level3/level4/level5"
    );
    let result = classifier.classify(&long_path, 100, 20);

    assert_eq!(result.contribution_type, ContributionType::ProductionCode);
    assert_eq!(result.language, Some("TypeScript (React)".to_string()));
}

#[test]
fn test_classify_handles_zero_lines_added_and_removed() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("src/main.rs", 0, 0);

    assert_eq!(result.contribution_type, ContributionType::ProductionCode);
    assert_eq!(result.lines_added, 0);
    assert_eq!(result.lines_removed, 0);
}

#[test]
fn test_classify_handles_large_line_counts() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("src/generated.ts", 10000, 5000);

    assert_eq!(result.contribution_type, ContributionType::ProductionCode);
    assert_eq!(result.lines_added, 10000);
    assert_eq!(result.lines_removed, 5000);
}

#[test]
fn test_classify_preserves_original_file_path_case() {
    let classifier = FileClassifier::new();
    let original_path = "SRC/MyComponent.TSX";
    let result = classifier.classify(original_path, 75, 15);

    // Even though classification is case-insensitive, original path should be preserved
    assert_eq!(result.file_path, original_path);
}

#[test]
fn test_classify_handles_windows_style_paths() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("C:\\Users\\dev\\project\\test_main.py", 30, 6);

    assert_eq!(result.contribution_type, ContributionType::Tests);
}

#[test]
fn test_classify_handles_paths_with_spaces() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("my folder/my file.test.js", 40, 8);

    assert_eq!(result.contribution_type, ContributionType::Tests);
}

#[test]
fn test_get_extension_handles_no_extension() {
    let result = FileClassifier::get_extension("README");
    assert_eq!(result, ".README");
}

#[test]
fn test_get_extension_handles_multiple_dots() {
    let result = FileClassifier::get_extension("component.test.js");
    assert_eq!(result, ".js");
}

#[test]
fn test_get_extension_handles_hidden_files() {
    let result = FileClassifier::get_extension(".gitignore");
    assert_eq!(result, ".gitignore");
}

#[test]
fn test_get_extension_handles_empty_string() {
    let result = FileClassifier::get_extension("");
    assert_eq!(result, ".");
}

// ============================================================================
// Comprehensive Integration Tests
// ============================================================================

#[test]
fn test_comprehensive_classification_pipeline() {
    let classifier = FileClassifier::new();

    // Test a complete classification pipeline
    let test_cases = vec![
        ("src/auth/login.rs", ContributionType::ProductionCode, Some("Rust".to_string())),
        ("tests/test_login.rs", ContributionType::Tests, Some("Rust".to_string())),
        ("README.md", ContributionType::Documentation, Some("Documentation".to_string())),
        ("package.json", ContributionType::SpecsConfig, Some("Configuration".to_string())),
        ("Dockerfile", ContributionType::Infrastructure, Some("Infrastructure".to_string())),
        ("styles/main.css", ContributionType::Styling, Some("CSS/Styling".to_string())),
        ("random.xyz", ContributionType::Other, None),
    ];

    for (path, expected_type, expected_lang) in test_cases {
        let result = classifier.classify(path, 10, 5);
        assert_eq!(
            result.contribution_type, expected_type,
            "Failed for path: {}", path
        );
        assert_eq!(
            result.language, expected_lang,
            "Failed language detection for path: {}", path
        );
    }
}

#[test]
fn test_file_classifier_default_trait() {
    let classifier1 = FileClassifier::new();
    let classifier2 = FileClassifier::default();

    // Both should produce identical results
    let result1 = classifier1.classify("test.py", 10, 5);
    let result2 = classifier2.classify("test.py", 10, 5);

    assert_eq!(result1.contribution_type, result2.contribution_type);
    assert_eq!(result1.language, result2.language);
}

// ============================================================================
// False Positive Tests - Verifying classifier correctly avoids false positives
// These files contain substrings like "test" but should NOT be classified as tests
// ============================================================================

#[test]
fn test_no_false_positive_testimonials_is_production_code() {
    // "testimonials.py" contains "test" substring but is production code
    // Classifier uses precise patterns like "test_" and "_test." not just "test"
    let classifier = FileClassifier::new();
    let result = classifier.classify("src/testimonials.py", 50, 10);

    // Correctly classified as ProductionCode (no false positive!)
    assert_eq!(result.contribution_type, ContributionType::ProductionCode);
}

#[test]
fn test_no_false_positive_contest_is_production_code() {
    // "contest.js" contains "test" substring but should be production code
    let classifier = FileClassifier::new();
    let result = classifier.classify("src/contest.js", 80, 16);

    assert_eq!(result.contribution_type, ContributionType::ProductionCode);
}

#[test]
fn test_no_false_positive_latest_is_config() {
    // "latest.json" contains "test" substring but should be config
    let classifier = FileClassifier::new();
    let result = classifier.classify("latest.json", 20, 4);

    assert_eq!(result.contribution_type, ContributionType::SpecsConfig);
}

#[test]
fn test_no_false_positive_attest_is_production_code() {
    // "attest.go" contains "test" substring but is production code
    let classifier = FileClassifier::new();
    let result = classifier.classify("src/attest.go", 60, 12);

    assert_eq!(result.contribution_type, ContributionType::ProductionCode);
}

#[test]
fn test_no_false_positive_document_processor_is_production_code() {
    // "document_processor.py" should be production code, not documentation
    let classifier = FileClassifier::new();
    let result = classifier.classify("src/document_processor.py", 100, 20);

    assert_eq!(result.contribution_type, ContributionType::ProductionCode);
}

#[test]
fn test_no_false_positive_manifest_is_config() {
    // "manifest.json" contains "man" but should be config, not documentation
    let classifier = FileClassifier::new();
    let result = classifier.classify("manifest.json", 30, 6);

    assert_eq!(result.contribution_type, ContributionType::SpecsConfig);
}

#[test]
fn test_no_false_positive_fastest_is_production_code() {
    // "fastest.rs" contains "test" substring but is production code
    let classifier = FileClassifier::new();
    let result = classifier.classify("src/algorithms/fastest.rs", 150, 30);

    assert_eq!(result.contribution_type, ContributionType::ProductionCode);
}

#[test]
fn test_no_false_positive_protest_is_production_code() {
    // "protest.ts" contains "test" substring but is production code
    let classifier = FileClassifier::new();
    let result = classifier.classify("src/protest.ts", 90, 18);

    assert_eq!(result.contribution_type, ContributionType::ProductionCode);
}

#[test]
fn test_no_false_positive_greatest_is_production_code() {
    // "greatest.py" contains "test" substring
    let classifier = FileClassifier::new();
    let result = classifier.classify("utils/greatest.py", 45, 9);

    assert_eq!(result.contribution_type, ContributionType::ProductionCode);
}

#[test]
fn test_no_false_positive_detest_is_production_code() {
    // "detest.rb" contains "test" substring
    let classifier = FileClassifier::new();
    let result = classifier.classify("lib/detest.rb", 35, 7);

    assert_eq!(result.contribution_type, ContributionType::ProductionCode);
}

// ============================================================================
// True Negative Tests - Files that correctly are NOT classified as certain types
// ============================================================================

#[test]
fn test_true_negative_regular_py_is_not_test() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("src/utils.py", 40, 8);

    assert_eq!(result.contribution_type, ContributionType::ProductionCode);
    assert_ne!(result.contribution_type, ContributionType::Tests);
}

#[test]
fn test_true_negative_main_rs_is_not_documentation() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("src/main.rs", 200, 40);

    assert_eq!(result.contribution_type, ContributionType::ProductionCode);
    assert_ne!(result.contribution_type, ContributionType::Documentation);
}

#[test]
fn test_true_negative_app_js_is_not_infrastructure() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("src/app.js", 120, 24);

    assert_eq!(result.contribution_type, ContributionType::ProductionCode);
    assert_ne!(result.contribution_type, ContributionType::Infrastructure);
}

#[test]
fn test_true_negative_component_tsx_is_not_styling() {
    let classifier = FileClassifier::new();
    let result = classifier.classify("components/Header.tsx", 80, 16);

    assert_eq!(result.contribution_type, ContributionType::ProductionCode);
    assert_ne!(result.contribution_type, ContributionType::Styling);
}
