'use strict';
const path = require('path');
const fs = require('fs');

let target_build_file = null;
if (process.argv[2]) {
    target_build_file = process.argv[2];
} else {
    console.log('コマンドライン引数にparselによるビルド対象のファイルが指定されていません');
    console.log('例)node create_parcel_json.js [target_build_file] [ENV] [output]');
    process.exit(1);
}

let target_env = null;
if (process.argv[3]) {
    target_env = process.argv[3];
} else {
    console.log('コマンドライン引数にparselによるビルド対象のENVが指定されていません');
    console.log('例)node create_parcel_json.js [target_build_file] [ENV] [output]');
    process.exit(1);
}

let output = null;
if (process.argv[4]) {
    output = process.argv[4];
} else {
    console.log('コマンドライン引数にparselによるビルドの出力先が指定されていません');
    console.log('例)node create_parcel_json.js [target_build_file] [ENV] [output]');
    process.exit(1);
}

if (!fs.existsSync(target_build_file)) {
    console.log(`${target_build_file}は存在しないファイルです。`);
    process.exit(1);
}

const app_prefix = target_build_file.match(/\w*\/index.html/)[0].replace(/\/index.html/, '');
const dist_env = `dist/${target_env}`;
const target_file = `${dist_env}/${app_prefix}/index.html`;
if (!fs.existsSync(target_file)) {
    console.log(`${target_file}は存在しないファイルです。`);
    process.exit(1);
}

const read_target_build_file = fs.readFileSync(target_build_file, 'utf-8');
const read_target_file = fs.readFileSync(target_file, 'utf-8');

const read_target_build_scripts = read_target_build_file.match(/<script src="\S*.js">/g);
const read_target_file_scripts = read_target_file.match(/<script src="\S*.js">/g);

let json = {};
read_target_build_scripts.forEach((target_build_script, index) => {
    target_build_script = target_build_script.replace(/<script src="/, '').replace(/">/, '');
    const read_target_file_script = read_target_file_scripts[index].replace(/<script src="\S*\//, '').replace(/">/, '');
    json[target_build_script.replace(/.js/, '_js')] = read_target_file_script;
    fs.copyFileSync(`${dist_env}/${app_prefix}/${read_target_file_script}`, `${output}/${read_target_file_script}`);

    const target_css = read_target_file_script.replace(/.js/, '.css');
    if (fs.existsSync(`${dist_env}/${app_prefix}/${target_css}`)) {
        json[target_build_script.replace(/.js/, '_css')] = target_css;
        fs.copyFileSync(`${dist_env}/${app_prefix}/${read_target_file_script}`, `${output}/${target_css}`);
    }
});
console.log(json);
fs.writeFileSync(`${output}/${app_prefix}.json`, JSON.stringify(json), 'utf8');
