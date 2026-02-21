use anyhow::Result;
use reqwest::Client;
use serde_json::{json, Value};
use tauri::AppHandle;

fn get_system_prompt(from: &str, to: &str, scene: &str, mode: &str, daily_mode: bool) -> String {
    // 语言代码映射到中文名称的辅助函数
    fn get_lang_name(code: &str) -> String {
        match code {
            "zh" => "中文".to_string(),
            "en" => "英文".to_string(),
            "ja" => "日文".to_string(),
            "ko" => "韩文".to_string(),
            "es" => "西班牙文".to_string(),
            "fr" => "法文".to_string(),
            "de" => "德文".to_string(),
            "ru" => "俄文".to_string(),
            _ => code.to_string(),
        }
    }

    if daily_mode {
        return format!(
            r#"<task>将用户输入从【{}】翻译到【{}】</task>
            <language_requirement>
                1. 必须用【{}】输出
                2. 禁止使用其他语言
            </language_requirement>
            <requirements>
                1. 直接输出翻译结果，禁止任何解释
                2. 单句长度控制在10-25字,必须不超过50字
                3. 保留数字和专有名词
            </requirements>
            <style>自然流畅不生硬</style>
            <output_format>仅输出一条最终翻译结果，不要包含任何思考过程或解释</output_format>"#,
            from, to, get_lang_name(to)
        );
    }

    let base = if from == to {
        // 同语言模式：进行转述和风格改写
        format!(
            r#"<task>用【{}】扩写用户输入的内容</task>
            <language_requirement>必须用【{}】输出，不许超过50字</language_requirement>"#,
            get_lang_name(to), get_lang_name(to)
        )
    } else {
        // 跨语言翻译模式
        format!(
            r#"<task>将游戏内文字从【{}】翻译到【{}】</task>
            <language_requirement>必须用【{}】输出，不许超过50字</language_requirement>"#,
            from, to, get_lang_name(to)
        )
    };

    let mode_desc = match mode {
        "toxic" => {
            r#"<toxic_style>
                • 翻译/改写用户原文后，还需进行简单扩写，不超过50字
                • 用最为地狱毒舌的心态来攻击对手
                • 融入游戏场景梗
                • 符号化敏感词（如f*ck/傻*/草nm）
            </toxic_style>
            <references>中文: 百度贴吧老哥风格,充满网络喷子式的毒鸡汤,游戏嘲讽</references>
            <rules>使用FPS/MOBA黑话重构</rules>"#
        }
        "pro" => {
            r#"<pro_style>
            • 赛事解说风格
            • 25字以内短句
            • 优先使用目标语言的正式术语
            • 添加战术标记（[推线]/[Gank]）
            </pro_style>

            <rhythm>
            • 0.5秒可读速度
            • 去除冗余修饰词
            </rhythm>"#
        }
        "auto" => match scene {
            "dota2" | "lol" => {
                r#"<moba_style>
                • 保留技能,装备,英雄等缩写
                • 使用MOBA游戏特有黑话
                • 不要额外增加英雄名称
                </moba_style>"#
            }
            "csgo" => {
                r#"<fps_style>
                • 使用FPS战术简称(A1、B2等)
                • 转换为标准报点格式
                • 保留英文武器代号
                • 使用经济术语(eco、force等)
                </fps_style>"#
            }
            _ => {
                r#"<general_style>
                • 识别并保留游戏术语
                • 转换为玩家间常用表达
                • 保持游戏交流的简洁性
                </general_style>"#
            }
        },
        _ => "",
    };

    let scene_desc = match scene {
        "dota2" => {
            r#"<context>
            • 环境: DOTA2
            • 保留技能,装备（如BKB）,英雄（如ES=撼地神牛）等缩写
            • 使用赛事解说术语（如“对线”、“推塔”、“团战”）
            </context>"#
        }
        "lol" => {
            r#"<context>
            • 英雄联盟游戏环境
            • 保留技能和装备简称
            • 使用赛事解说术语
            </context>"#
        }
        "csgo" => {
            r#"<context>
            • CS:GO游戏环境
            • 保留武器和位置代号
            • 使用标准战术用语
            </context>"#
        }
        _ => {
            r#"<context>
            • 通用游戏环境
            • 识别常见游戏用语
            • 保持游戏交流特点
            </context>"#
        }
    };

    format!(
        r#"{}{}{}
        <compliance>
        • 严格长度不超过50字
        • 敏感词二次过滤
        </compliance>
        <output_format>仅输出一条最终翻译结果，不要包含任何思考过程或解释</output_format>"#,
        base, mode_desc, scene_desc
    )
}

fn get_model_config(settings: &crate::store::AppSettings) -> crate::store::ModelConfig {
    use std::env;
    
    match settings.model_type.as_str() {
        "deepseek" => crate::store::ModelConfig {
            auth: env::var("DEEPSEEK_API_KEY")
                .expect("请在 .env 文件中设置 DEEPSEEK_API_KEY"),
            api_url: "https://api-inference.modelscope.cn/v1/chat/completions".to_string(),
            model_name: "deepseek-ai/DeepSeek-V3.2".to_string(),
        },
        "DeepSeek-OCR" => crate::store::ModelConfig {
            auth: env::var("DEEPSEEK_OCR_API_KEY")
                .expect("请在 .env 文件中设置 DEEPSEEK_OCR_API_KEY"),
            api_url: "https://api.siliconflow.cn/v1/chat/completions".to_string(),
            model_name: "deepseek-ai/DeepSeek-OCR".to_string(),
        },
        "GLM" => crate::store::ModelConfig {
            auth: env::var("GLM_API_KEY")
                .expect("请在 .env 文件中设置 GLM_API_KEY"),
            api_url: "https://open.bigmodel.cn/api/paas/v4/chat/completions".to_string(),
            model_name: "glm-4.7-flash".to_string(),
        },
        "custom" => settings.custom_model.clone(),
        _ => settings.custom_model.clone(),
    }
}

pub async fn translate_with_gpt(app: &AppHandle, original: &str) -> Result<String> {
    let settings = crate::store::get_settings(app)?;
    println!("当前翻译设置:");
    println!("- 源语言: {}", settings.translation_from);
    println!("- 目标语言: {}", settings.translation_to);
    println!("- 游戏场景: {}", settings.game_scene);
    println!("- 翻译模式: {}", settings.translation_mode);
    println!("- 日常模式: {}", settings.daily_mode);
    println!("- 模型类型: {}", settings.model_type);

    let model_config = get_model_config(&settings);

    println!("正在发送请求到: {}", model_config.api_url);
    println!("使用的模型: {}", model_config.model_name);
    println!("模型API密钥前缀: {}", &model_config.auth[..std::cmp::min(12, model_config.auth.len())]);

    let system_prompt = get_system_prompt(
        &settings.translation_from,
        &settings.translation_to,
        &settings.game_scene,
        &settings.translation_mode,
        settings.daily_mode,
    );

    println!("\n📋 生成的系统提示词:\n{}\n", system_prompt);

    let client = Client::new();

    let request_body = match settings.model_type.as_str() {
        "GLM" => json!({
            "model": model_config.model_name,
            "messages": [
                {
                    "role": "assistant",
                    "content": system_prompt
                },
                {
                    "role": "user",
                    "content": original
                }
            ],
            "max_tokens": 8000
        }),
        _ => json!({
            "model": model_config.model_name,
            "messages": [
                {
                    "role": "system",
                    "content": system_prompt
                },
                {
                    "role": "user",
                    "content": original
                }
            ],
            "max_tokens": 300,
            "temperature": 0.9,
            "top_p": 0.7,
            "n": 1,
            "stream": false,
            "presence_penalty": 0.3,
            "frequency_penalty": -0.3
        }),
    };

    let response = match client
        .post(&model_config.api_url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", model_config.auth))
        .json(&request_body)
        .send()
        .await
    {
        Ok(resp) => match resp.json::<Value>().await {
            Ok(json) => {
                // 先检查是否有错误信息
                if let Some(error) = json.get("error_msg").and_then(|msg| msg.as_str()) {
                    println!("API返回错误: {}", error);
                    return Ok(format!("[错误] {}", error));
                }
                json
            }
            Err(e) => {
                println!("解析响应JSON失败: {}", e);
                return Ok(format!("[错误] 服务器响应格式异常: {}", e));
            }
        },
        Err(e) => {
            let error_msg = match e.to_string().as_str() {
                msg if msg.contains("connection refused") => "无法连接到API服务器，请检查网络设置",
                msg if msg.contains("timeout") => "请求超时，请检查网络连接",
                msg if msg.contains("certificate") => "SSL证书验证失败，请检查网络设置",
                _ => "网络请求失败",
            };
            println!("请求失败: {}", e);
            return Ok(format!("[错误] {}", error_msg));
        }
    };

    // 解析响应
    println!("API响应原文: {:?}", response);
    let translated = match response
        .get("choices")
        .and_then(|choices| choices.as_array())
        .and_then(|choices| choices.first())
        .and_then(|choice| choice.get("message"))
        .and_then(|message| message.get("content"))
        .and_then(|content| content.as_str())
    {
        Some(text) => {
            let text = text.trim();
            // 如果找到</think>标签，只保留其后内容
            if let Some(end_pos) = text.find("</think>") {
                text[(end_pos + 8)..].trim().to_string()
            } else {
                text.to_string()
            }
        }
        None => {
            println!("无法从响应中提取翻译结果: {:?}", response);
            return Ok("[错误] 服务器返回的数据格式异常".to_string());
        }
    };

    Ok(translated)
}


/// 测试专用：直接调用翻译 API，不依赖 AppHandle
pub async fn test_translate(
    model_type: &str,
    original: &str,
    from: &str,
    to: &str,
) -> Result<String> {
    use serde_json::Value;
    use std::env;
    
    // 获取模型配置
    let model_config = match model_type {
        "testmodel" => crate::store::ModelConfig {
            auth: env::var("DEEPSEEK_API_KEY")
                .expect("请在 .env 文件中设置 DEEPSEEK_API_KEY"),
            api_url: "https://api-inference.modelscope.cn/v1/chat/completions".to_string(),
            model_name: "deepseek-ai/DeepSeek-V3.2".to_string(),
        },
        _ => return Err(anyhow::anyhow!("未知的模型类型: {}", model_type)),
    };

    println!("========================================");
    println!("测试配置: 模型={}, API={}", model_type, model_config.api_url);
    println!("模型名称: {}", model_config.model_name);
    println!("原文: {}", original);
    println!("翻译方向: {} -> {}", from, to);
    println!("========================================");
    
    let system_prompt = get_system_prompt(from, to, "dota2", "toxic", false);

    let client = Client::new();
    let request_body = json!({
        "model": model_config.model_name,
        "messages": [
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": original}
        ],
        "max_tokens": 300,
        "temperature": 0.9,
        "top_p": 0.7,
        "n": 1,
        "stream": false
    });

    println!("📤 发送请求...");
    
    let response_result = client
        .post(&model_config.api_url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", model_config.auth))
        .json(&request_body)
        .send()
        .await;

    let response = match response_result {
        Ok(resp) => {
            let status = resp.status();
            println!("📥 HTTP 状态码: {}", status);
            
            if !status.is_success() {
                let error_text = resp.text().await?;
                println!("❌ 错误响应: {}", error_text);
                return Err(anyhow::anyhow!("API 返回错误状态: {}", status));
            }
            
            resp
        }
        Err(e) => {
            println!("❌ 请求失败: {}", e);
            return Err(anyhow::anyhow!("网络请求失败: {}", e));
        }
    };

    let json_result = response.json::<Value>().await;
    
    let json = match json_result {
        Ok(j) => {
            println!("\n📋 完整 API 响应:");
            println!("{}", serde_json::to_string_pretty(&j).unwrap_or_else(|_| format!("{:?}", j)));
            println!();
            j
        }
        Err(e) => {
            println!("❌ JSON 解析失败: {}", e);
            return Err(anyhow::anyhow!("无法解析响应为 JSON: {}", e));
        }
    };

    // 检查是否有错误信息
    if let Some(error) = json.get("error") {
        println!("❌ API 返回错误: {:?}", error);
        return Err(anyhow::anyhow!("API 错误: {:?}", error));
    }

    // 尝试提取翻译结果
    let translated = json
        .get("choices")
        .and_then(|choices| choices.as_array())
        .and_then(|arr| arr.first())
        .and_then(|choice| choice.get("message"))
        .and_then(|message| message.get("content"))
        .and_then(|content| content.as_str())
        .ok_or_else(|| {
            println!("❌ 无法从响应中提取内容");
            println!("JSON 结构: {:?}", json);
            anyhow::anyhow!("响应格式不符合预期，无法提取翻译结果")
        })?;

    let translated = translated.trim();
    
    // 处理 DeepSeek-R1 的 <think> 标签
    let result = if let Some(pos) = translated.find("</think>") {
        translated[pos + 8..].trim().to_string()
    } else {
        translated.to_string()
    };

    println!("========================================");
    println!("✅ 翻译成功!");
    println!("译文: {}", result);
    println!("========================================");

    Ok(result)
}