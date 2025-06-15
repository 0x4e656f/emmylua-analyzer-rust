<div align="center">

# 🔧 EmmyLua Configuration Guide

*Comprehensive guide to EmmyLua Analyzer Rust configuration options*

[![Back to Main](https://img.shields.io/badge/← Back to Main-blue?style=for-the-badge)](../../README.md)

</div>

---

## 📋 Overview

EmmyLua language server supports a flexible configuration system that allows fine-grained control over various features through configuration files.

### 📁 Configuration Files

<table>
<tr>
<td width="50%">

#### 📄 **Main Configuration File**
- **`.emmyrc.json`**: Main configuration file
- **Location**: Project root directory
- **Priority**: Highest

</td>
<td width="50%">

#### 🔄 **Compatibility Configuration**
- **`.luarc.json`**: Compatibility configuration file
- **Auto Conversion**: Converts to `.emmyrc.json` format
- **Override Rules**: Overridden by `.emmyrc.json`

</td>
</tr>
</table>

> **💡 Note**: `.emmyrc.json` configuration format is more feature-rich, and incompatible parts will be automatically ignored.

### 🛠️ Schema Support

To enable intelligent completion and validation for configuration files, you can add a schema reference to your configuration file:

```json
{
  "$schema": "https://github.com/CppCXY/emmylua-analyzer-rust/blob/main/crates/emmylua_code_analysis/resources/schema.json"
}
```

---

## 📝 Complete Configuration Example

Here's a complete configuration file example containing all configuration options:

<details>
<summary><b>Click to expand complete configuration</b></summary>

```json
{
  "$schema": "https://github.com/CppCXY/emmylua-analyzer-rust/blob/main/crates/emmylua_code_analysis/resources/schema.json",
  "completion": {
    "enable": true,
    "autoRequire": true,
    "autoRequireFunction": "require",
    "autoRequireNamingConvention": "keep",
    "callSnippet": false,
    "postfix": "@"
  },
  "signature": {
    "detailSignatureHelper": false
  },
  "diagnostics": {
    "disable": [],
    "globals": [],
    "globalsRegex": [],
    "severity": {},
    "enables": []
  },
  "hint": {
    "enable": true,
    "paramHint": true,
    "indexHint": true,
    "localHint": true,
    "overrideHint": true
  },
  "runtime": {
    "version": "Lua5.4",
    "requireLikeFunction": [],
    "frameworkVersions": [],
    "extensions": [],
    "requirePattern": []
  },
  "workspace": {
    "ignoreDir": [],
    "ignoreGlobs": [],
    "library": [],
    "workspaceRoots": [],
    "encoding": "",
    "moduleMap": [],
    "reindexDuration": 5000
  },
  "resource": {
    "paths": []
  },
  "codeLens": {
    "enable": true
  },
  "strict": {
    "requirePath": false,
    "typeCall": false,
    "arrayIndex": false,
    "metaOverrideFileDefine": true
  },
  "hover": {
    "enable": true
  },
  "references": {
    "enable": true,
    "fuzzy_search": true
  }
}
```

</details>

---

## 🎯 Configuration Categories Explained

### 💡 completion - Code Completion

<div align="center">

#### Intelligent completion configuration for enhanced coding efficiency

</div>

| Configuration | Type | Default | Description |
|---------------|------|---------|-------------|
| **`enable`** | `boolean` | `true` | 🔧 Enable/disable code completion features |
| **`autoRequire`** | `boolean` | `true` | 📦 Auto-complete require statements |
| **`autoRequireFunction`** | `string` | `"require"` | ⚡ Function name used for auto-completion |
| **`autoRequireNamingConvention`** | `string` | `"keep"` | 🏷️ Naming convention conversion method |
| **`callSnippet`** | `boolean` | `false` | 🎪 Enable function call snippets |
| **`postfix`** | `string` | `"@"` | 🔧 Postfix completion trigger symbol |

#### 🏷️ Naming Convention Options

<table>
<tr>
<td width="25%">

**`keep`**  
Keep original

</td>
<td width="25%">

**`camel-case`**  
Camel case

</td>
<td width="25%">

**`snake-case`**  
Snake case

</td>
<td width="25%">

**`pascal-case`**  
Pascal case

</td>
</tr>
</table>

---

### 📝 signature - Function Signature

| Configuration | Type | Default | Description |
|---------------|------|---------|-------------|
| **`detailSignatureHelper`** | `boolean` | `false` | 📊 Show detailed function signature help (currently inactive) |

---

### 🔍 diagnostics - Code Diagnostics

<div align="center">

#### Powerful static analysis and error detection system

</div>

| Configuration | Type | Default | Description |
|---------------|------|---------|-------------|
| **`disable`** | `string[]` | `[]` | ❌ List of disabled diagnostic messages |
| **`globals`** | `string[]` | `[]` | 🌐 Global variable whitelist |
| **`globalsRegex`** | `string[]` | `[]` | 🔤 Global variable regex patterns |
| **`severity`** | `object` | `{}` | ⚠️ Diagnostic message severity configuration |
| **`enables`** | `string[]` | `[]` | ✅ List of enabled diagnostic messages |

#### 🎯 Severity Levels

<table>
<tr>
<td width="25%">

**`error`**  
🔴 Error

</td>
<td width="25%">

**`warning`**  
🟡 Warning

</td>
<td width="25%">

**`information`**  
🔵 Information

</td>
<td width="25%">

**`hint`**  
💡 Hint

</td>
</tr>
</table>

#### 📋 Common Diagnostic Message Examples

```json
{
  "diagnostics": {
    "disable": ["undefined-global"],
    "severity": {
      "undefined-global": "warning",
      "unused-local": "hint"
    },
    "enables": ["undefined-field"]
  }
}
```

---

### 💡 hint - Inline Hints

<div align="center">

#### Intelligent inline hint system for viewing type information without mouse hover

</div>

| Configuration | Type | Default | Description |
|---------------|------|---------|-------------|
| **`enable`** | `boolean` | `true` | 🔧 Enable/disable inline hints |
| **`paramHint`** | `boolean` | `true` | 🏷️ Show function parameter hints |
| **`indexHint`** | `boolean` | `true` | 📊 Show cross-line index expression hints |
| **`localHint`** | `boolean` | `true` | 📍 Show local variable type hints |
| **`overrideHint`** | `boolean` | `true` | 🔄 Show method override hints |

---

### ⚙️ runtime - Runtime Environment

<div align="center">

#### Configure Lua runtime environment and version features

</div>

| Configuration | Type | Default | Description |
|---------------|------|---------|-------------|
| **`version`** | `string` | `"Lua5.4"` | 🚀 Lua version selection |
| **`requireLikeFunction`** | `string[]` | `[]` | 📦 List of require-like functions |
| **`frameworkVersions`** | `string[]` | `[]` | 🎯 Framework version identifiers |
| **`extensions`** | `string[]` | `[]` | 📄 Supported file extensions |
| **`requirePattern`** | `string[]` | `[]` | 🔍 Require pattern matching rules |

#### 🚀 Supported Lua Versions

<table>
<tr>
<td width="20%">

**`Lua5.1`**  
Classic version

</td>
<td width="20%">

**`Lua5.2`**  
Enhanced features

</td>
<td width="20%">

**`Lua5.3`**  
Integer support

</td>
<td width="20%">

**`Lua5.4`**  
Latest features

</td>
<td width="20%">

**`LuaJIT`**  
High performance

</td>
</tr>
</table>

#### 📋 Runtime Configuration Example

```json
{
  "runtime": {
    "version": "Lua5.4",
    "requireLikeFunction": ["import", "load"],
    "frameworkVersions": ["love2d", "openresty"],
    "extensions": [".lua", ".lua.txt"],
    "requirePattern": ["?.lua", "?/init.lua"]
  }
}
```

---

### 🏗️ workspace - Workspace Configuration

<div align="center">

#### Workspace and project structure configuration, supporting both relative and absolute paths

</div>

| Configuration | Type | Default | Description |
|---------------|------|---------|-------------|
| **`ignoreDir`** | `string[]` | `[]` | 📁 List of directories to ignore |
| **`ignoreGlobs`** | `string[]` | `[]` | 🔍 Glob pattern-based file ignores |
| **`library`** | `string[]` | `[]` | 📚 Library directory paths |
| **`workspaceRoots`** | `string[]` | `[]` | 🏠 Workspace root directory list |
| **`encoding`** | `string` | `"utf-8"` | 🔤 File encoding format |
| **`moduleMap`** | `object[]` | `[]` | 🗺️ Module path mapping rules |
| **`reindexDuration`** | `number` | `5000` | ⏱️ Reindexing time interval (milliseconds) |

#### 🗺️ Module Mapping Configuration

Module mapping is used to transform one module path to another, supporting regular expressions:

```json
{
  "workspace": {
    "moduleMap": [
      {
        "pattern": "^lib(.*)$",
        "replace": "script$1"
      }
    ]
  }
}
```

#### 📋 Workspace Configuration Example

```json
{
  "workspace": {
    "ignoreDir": ["build", "dist", "node_modules"],
    "ignoreGlobs": ["*.log", "*.tmp", "test_*"],
    "library": ["/usr/local/lib/lua", "./libs"],
    "workspaceRoots": ["Assets/Scripts/Lua"],
    "encoding": "utf-8",
    "reindexDuration": 3000
  }
}
```

---

### 📁 resource - Resource Paths

| Configuration | Type | Default | Description |
|---------------|------|---------|-------------|
| **`paths`** | `string[]` | `[]` | 🎯 Resource file root directory list |

> **💡 Purpose**: Configuring resource directories allows EmmyLua to properly provide file path completion and navigation features.

---

### 👁️ codeLens - Code Lens

| Configuration | Type | Default | Description |
|---------------|------|---------|-------------|
| **`enable`** | `boolean` | `true` | 🔍 Enable/disable CodeLens features |

---

### 🔒 strict - Strict Mode

<div align="center">

#### Strict mode configuration, controlling the strictness of type checking and code analysis

</div>

| Configuration | Type | Default | Description |
|---------------|------|---------|-------------|
| **`requirePath`** | `boolean` | `false` | 📍 Require path strict mode |
| **`typeCall`** | `boolean` | `false` | 🎯 Type call strict mode |
| **`arrayIndex`** | `boolean` | `false` | 📊 Array index strict mode |
| **`metaOverrideFileDefine`** | `boolean` | `true` | 🔄 Meta definitions override file definitions |

#### 🎯 Strict Mode Explanation

<table>
<tr>
<td width="50%">

**🔒 When Strict Mode is Enabled**
- **require path**: Must start from specified root directory
- **type call**: Must manually define overloads
- **array index**: Strict index rule compliance
- **meta definitions**: Override definitions in files

</td>
<td width="50%">

**🔓 When Strict Mode is Disabled**
- **require path**: Flexible path resolution
- **type call**: Returns self type
- **array index**: Lenient index checking
- **meta definitions**: Behaves like `luals`

</td>
</tr>
</table>

---

### 👁️ hover - Hover Hints

| Configuration | Type | Default | Description |
|---------------|------|---------|-------------|
| **`enable`** | `boolean` | `true` | 🖱️ Enable/disable mouse hover hints |

---

### 🔗 references - Reference Finding

| Configuration | Type | Default | Description |
|---------------|------|---------|-------------|
| **`enable`** | `boolean` | `true` | 🔍 Enable/disable reference finding features |
| **`fuzzy_search`** | `boolean` | `true` | 🎯 Enable fuzzy search |

---

<div align="center">

## 🎯 Summary

By properly configuring EmmyLua, you can:

- **🎯 Enhance Development Efficiency**: Intelligent completion and hints
- **🔍 Improve Code Quality**: Strict type checking and diagnostics
- **🛠️ Customize Development Environment**: Adapt to different project needs
- **⚡ Optimize Performance**: Reasonable workspace and indexing configuration

[⬆ Back to Top](#-emmylua-configuration-guide)

</div>
