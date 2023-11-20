#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_playbook_yml_format_1_e1cff8b2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["yaml"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("---\n- hosts: webservers\n  vars:\n    http_port: 80\n    max_clients: 200\n  remote_user: root\n  tasks:\n  - name: ensure apache is at the latest version\n    yum:\n      name: httpd\n      state: latest\n  - name: write the apache config file\n    template:\n      src: /srv/httpd.j2\n      dest: /etc/httpd.conf\n    notify:\n    - restart apache\n  - name: ensure apache is running\n    service:\n      name: httpd\n      state: started\n  handlers:\n    - name: restart apache\n      service:\n        name: httpd\n        state: restarted") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "---\n- hosts: webservers\n  vars:\n    http_port: 80\n    max_clients: 200\n  remote_user: root\n  tasks:\n    - name: ensure apache is at the latest version\n      yum:\n        name: httpd\n        state: latest\n    - name: write the apache config file\n      template:\n        src: /srv/httpd.j2\n        dest: /etc/httpd.conf\n      notify:\n        - restart apache\n    - name: ensure apache is running\n      service:\n        name: httpd\n        state: started\n  handlers:\n    - name: restart apache\n      service:\n        name: httpd\n        state: restarted");
}
