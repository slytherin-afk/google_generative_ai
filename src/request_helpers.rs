use crate::{
    Content, ContentElement, GenerateContentPrompt, GenerateContentRequest, Part, TextPart,
};

pub(crate) fn format_generate_content_input(
    prompt: GenerateContentPrompt,
) -> anyhow::Result<GenerateContentRequest> {
    let content = match prompt {
        GenerateContentPrompt::Text(text) => Content {
            parts: vec![Part::Text(TextPart { text })],
            role: Some("user".to_string()),
        },
        GenerateContentPrompt::FullRequest(request) => return Ok(request),
        GenerateContentPrompt::List(list) => {
            assign_role_to_parts_and_validate_send_message_request(list)?
        }
    };

    Ok(GenerateContentRequest {
        contents: vec![content],
        generation_config: None,
        safety_settings: None,
        system_instruction: None,
        tools: None,
        tool_config: None,
    })
}

fn assign_role_to_parts_and_validate_send_message_request(
    parts: Vec<ContentElement>,
) -> anyhow::Result<Content> {
    let (has_user_content, has_function_content, mut content) = parts.into_iter().fold(
        (
            false,
            false,
            Content {
                parts: vec![],
                role: None,
            },
        ),
        |(has_user_content, has_function_content, mut content), part| match part {
            ContentElement::Text(text) => {
                content.parts.push(Part::Text(TextPart { text }));
                (true, has_function_content, content)
            }
            ContentElement::Part(part) => match part {
                Part::FunctionResponse(_) => {
                    content.parts.push(part);
                    (has_user_content, true, content)
                }
                _ => {
                    content.parts.push(part);
                    (true, has_function_content, content)
                }
            },
        },
    );

    if has_user_content && has_function_content {
        anyhow::bail!("Within a single message, FunctionResponse cannot be mixed with other type of part in the request for sending chat message.");
    }

    if !has_user_content && !has_function_content {
        anyhow::bail!("No content is provided for sending chat message.");
    }

    content.role = Some(if has_user_content { "user" } else { "function" }.to_string());

    Ok(content)
}
