# Quick Task Plan

## Task
修改 `.github/workflows/release.yml` 以支持自动创建 TAG，自动上传到 release

## Current State
- 工作流当前只在 push tag 时触发 (`on.push.tags: - "v*"`)
- 需要手动创建 tag 才能触发 release

## Changes Required
1. 添加 workflow_dispatch 触发器，支持手动触发
2. 添加自动 tag 创建步骤（当手动触发时）
3. 保持原有 push tag 触发逻辑不变

## Implementation

### 修改点 1: 添加 workflow_dispatch 触发器
```yaml
on:
  push:
    tags:
      - "v*"
  workflow_dispatch:
    inputs:
      version:
        description: 'Version number (e.g., 1.2.3)'
        required: true
        type: string
```

### 修改点 2: 添加自动 tag 创建 job
- 在 create-release job 之前添加 create-tag job
- 当 workflow_dispatch 触发时，自动创建 tag
- 将 tag 信息传递给后续 jobs

### 修改点 3: 修改版本获取逻辑
- 支持从 input 获取版本（workflow_dispatch）
- 支持从 tag 获取版本（push）

## Files to Modify
- `.github/workflows/release.yml`
