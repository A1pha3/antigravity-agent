---
title: 项目概览
description: Antigravity Agent 项目的基本介绍和特性说明
category: getting-started
language: zh-CN
version: 1.0.3
lastUpdated: 2025-12-04
tags: [概览, 入门]
relatedDocs:
  - installation.md
  - quickstart.md
---

# Antigravity Agent 项目概览

## 概述

Antigravity Agent 是一个开箱即用的 Antigravity 账户管理程序，旨在简化多账户管理、配置备份和进程控制等操作。

本文档将为您介绍项目的基本信息、主要功能和技术架构。

## 目录

- [项目目的](#项目目的)
- [主要功能](#主要功能)
- [技术栈](#技术栈)
- [系统要求](#系统要求)
- [下一步](#下一步)

## 项目目的

Antigravity Agent 的设计目标是：

- **简化账户管理**: 提供直观的界面管理多个 Antigravity 账户
- **安全备份**: 支持配置的加密导入导出，保护用户数据
- **进程控制**: 便捷地启动、停止和监控 Antigravity 进程
- **跨平台支持**: 在 Windows、macOS 和 Linux 上提供一致的体验

## 主要功能

### 🔐 账户管理

- 多账户支持和快速切换
- 新账户登录和配置
- 账户信息查看和管理

### 💾 配置备份

- 配置导出（支持密码加密）
- 配置导入（支持密码解密）
- 备份文件管理

### 🚀 进程管理

- Antigravity 进程启动和停止
- 进程状态实时监控
- 自动重启和错误恢复

### 📊 数据库监控

- SQLite 数据库实时监控
- 数据变化通知
- 数据库状态查看

### 🎯 系统托盘

- 后台运行支持
- 快速访问常用功能
- 系统启动时自动运行

### 📝 日志系统

- 双层日志（控制台 + 文件）
- 日志级别配置
- 日志查看和导出

## 技术栈

### 前端技术

- **React 18**: 现代化的 UI 框架
- **TypeScript 5**: 类型安全的 JavaScript
- **Vite 7**: 快速的构建工具
- **Tailwind CSS 3**: 实用优先的 CSS 框架
- **Radix UI**: 无障碍的 UI 组件库
- **Zustand**: 轻量级状态管理

### 后端技术

- **Rust**: 高性能、内存安全的系统编程语言
- **Tauri 2**: 轻量级的桌面应用框架
- **SQLite**: 嵌入式数据库

### 开发工具

- **ESLint**: JavaScript/TypeScript 代码检查
- **Prettier**: 代码格式化
- **Cargo**: Rust 包管理器

## 系统要求

### Windows

- Windows 10 或更高版本
- 64 位操作系统

### macOS

- macOS 10.15 (Catalina) 或更高版本
- Intel 或 Apple Silicon 处理器

### Linux

- 现代 Linux 发行版（Ubuntu 20.04+, Fedora 35+, 等）
- 64 位操作系统
- 注：Linux 支持目前处于计划阶段

## 下一步

- 📥 [安装应用](installation.md) - 在您的系统上安装 Antigravity Agent
- 🚀 [快速开始](quickstart.md) - 5 分钟快速上手教程
- 📖 [使用手册](../user-guide/user-guide.md) - 了解所有功能的详细使用方法

## 相关文档

- [安装指南](installation.md)
- [快速开始](quickstart.md)
- [系统架构](../development/architecture.md)
