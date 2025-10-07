//! # Default Commands
//!
//! The command-bar commands are set up here, and iamb-specific commands are defined here. See
//! [modalkit::env::vim::command] for additional Vim commands we pull in.
use std::{convert::TryFrom, str::FromStr as _};

use matrix_sdk::ruma::{events::tag::TagName, OwnedRoomId, OwnedUserId};

use modalkit::{
    commands::{CommandError, CommandResult, CommandStep},
    env::vim::command::{CommandContext, CommandDescription, OptionType},
    prelude::OpenTarget,
};

use crate::base::{
    CreateRoomFlags,
    CreateRoomType,
    DownloadFlags,
    HomeserverAction,
    IambAction,
    IambId,
    KeysAction,
    MemberUpdateAction,
    MessageAction,
    ProgramCommand,
    ProgramCommands,
    RoomAction,
    RoomField,
    SendAction,
    SpaceAction,
    VerifyAction,
};

type ProgContext = CommandContext;
type ProgResult = CommandResult<ProgramCommand>;

/// Convert strings the user types into a tag name.
fn tag_name(name: String) -> Result<TagName, CommandError> {
    let tag = match name.to_lowercase().as_str() {
        "fav" | "favorite" | "favourite" | "m.favourite" => TagName::Favorite,
        "low" | "lowpriority" | "low_priority" | "low-priority" | "m.lowpriority" => {
            TagName::LowPriority
        },
        "servernotice" | "server_notice" | "server-notice" | "m.server_notice" => {
            TagName::ServerNotice
        },
        _ => {
            if let Ok(tag) = name.parse() {
                TagName::User(tag)
            } else {
                let msg = format!("Invalid user tag name: {name}");

                return Err(CommandError::Error(msg));
            }
        },
    };

    Ok(tag)
}

fn iamb_invite(desc: CommandDescription, ctx: &mut ProgContext) -> ProgResult {
    let args = desc.arg.strings()?;

    if args.is_empty() {
        return Err(CommandError::InvalidArgument);
    }

    let ract = match args[0].as_str() {
        "accept" => {
            if args.len() != 1 {
                return Err(CommandError::InvalidArgument);
            }

            RoomAction::InviteAccept
        },
        "reject" => {
            if args.len() != 1 {
                return Err(CommandError::InvalidArgument);
            }

            RoomAction::InviteReject
        },
        "send" => {
            if args.len() != 2 {
                return Err(CommandError::InvalidArgument);
            }

            if let Ok(user) = OwnedUserId::try_from(args[1].as_str()) {
                RoomAction::InviteSend(user)
            } else {
                let msg = format!("Invalid user identifier: {}", args[1]);
                let err = CommandError::Error(msg);

                return Err(err);
            }
        },
        _ => {
            return Err(CommandError::InvalidArgument);
        },
    };

    let iact = IambAction::from(ract);
    let step = CommandStep::Continue(iact.into(), ctx.context.clone());

    return Ok(step);
}

fn iamb_keys(desc: CommandDescription, ctx: &mut ProgContext) -> ProgResult {
    let mut args = desc.arg.strings()?;

    if args.len() != 3 {
        return Err(CommandError::InvalidArgument);
    }

    let act = args.remove(0);
    let path = args.remove(0);
    let passphrase = args.remove(0);

    let act = match act.as_str() {
        "export" => KeysAction::Export(path, passphrase),
        "import" => KeysAction::Import(path, passphrase),
        _ => return Err(CommandError::InvalidArgument),
    };

    let vact = IambAction::Keys(act);
    let step = CommandStep::Continue(vact.into(), ctx.context.clone());

    return Ok(step);
}

fn iamb_verify(desc: CommandDescription, ctx: &mut ProgContext) -> ProgResult {
    let mut args = desc.arg.strings()?;

    match args.len() {
        0 => {
            let open = ctx.switch(OpenTarget::Application(IambId::VerifyList));
            let step = CommandStep::Continue(open, ctx.context.clone());

            return Ok(step);
        },
        1 => {
            return Result::Err(CommandError::InvalidArgument);
        },
        2 => {
            let act = match args[0].as_str() {
                "accept" => VerifyAction::Accept,
                "cancel" => VerifyAction::Cancel,
                "confirm" => VerifyAction::Confirm,
                "mismatch" => VerifyAction::Mismatch,
                "request" => {
                    let iact = IambAction::VerifyRequest(args.remove(1));
                    let step = CommandStep::Continue(iact.into(), ctx.context.clone());

                    return Ok(step);
                },
                _ => return Result::Err(CommandError::InvalidArgument),
            };

            let vact = IambAction::Verify(act, args.remove(1));
            let step = CommandStep::Continue(vact.into(), ctx.context.clone());

            return Ok(step);
        },
        _ => {
            return Result::Err(CommandError::InvalidArgument);
        },
    }
}

fn iamb_dms(desc: CommandDescription, ctx: &mut ProgContext) -> ProgResult {
    if !desc.arg.text.is_empty() {
        return Result::Err(CommandError::InvalidArgument);
    }

    let open = ctx.switch(OpenTarget::Application(IambId::DirectList));
    let step = CommandStep::Continue(open, ctx.context.clone());

    return Ok(step);
}

fn iamb_members(desc: CommandDescription, ctx: &mut ProgContext) -> ProgResult {
    if !desc.arg.text.is_empty() {
        return Result::Err(CommandError::InvalidArgument);
    }

    let open = IambAction::Room(RoomAction::Members(ctx.clone().into()));
    let step = CommandStep::Continue(open.into(), ctx.context.clone());

    return Ok(step);
}

fn iamb_leave(desc: CommandDescription, ctx: &mut ProgContext) -> ProgResult {
    if !desc.arg.text.is_empty() {
        return Result::Err(CommandError::InvalidArgument);
    }

    let leave = IambAction::Room(RoomAction::Leave(desc.bang));
    let step = CommandStep::Continue(leave.into(), ctx.context.clone());

    return Ok(step);
}

fn iamb_cancel(desc: CommandDescription, ctx: &mut ProgContext) -> ProgResult {
    if !desc.arg.text.is_empty() {
        return Result::Err(CommandError::InvalidArgument);
    }

    let mact = IambAction::from(MessageAction::Cancel(desc.bang));
    let step = CommandStep::Continue(mact.into(), ctx.context.clone());

    return Ok(step);
}

fn iamb_edit(desc: CommandDescription, ctx: &mut ProgContext) -> ProgResult {
    if !desc.arg.text.is_empty() {
        return Result::Err(CommandError::InvalidArgument);
    }

    let mact = IambAction::from(MessageAction::Edit);
    let step = CommandStep::Continue(mact.into(), ctx.context.clone());

    return Ok(step);
}

fn iamb_react(desc: CommandDescription, ctx: &mut ProgContext) -> ProgResult {
    let mut args = desc.arg.strings()?;

    if args.len() != 1 {
        return Result::Err(CommandError::InvalidArgument);
    }

    let react = args.remove(0);
    let mact = IambAction::from(MessageAction::React(react, desc.bang));
    let step = CommandStep::Continue(mact.into(), ctx.context.clone());

    return Ok(step);
}

fn iamb_unreact(desc: CommandDescription, ctx: &mut ProgContext) -> ProgResult {
    let mut args = desc.arg.strings()?;

    if args.len() > 1 {
        return Result::Err(CommandError::InvalidArgument);
    }

    let reaction = args.pop();
    let mact = IambAction::from(MessageAction::Unreact(reaction, desc.bang));
    let step = CommandStep::Continue(mact.into(), ctx.context.clone());

    return Ok(step);
}

fn iamb_redact(desc: CommandDescription, ctx: &mut ProgContext) -> ProgResult {
    let args = desc.arg.strings()?;

    if args.len() > 1 {
        return Result::Err(CommandError::InvalidArgument);
    }

    let reason = args.into_iter().next();
    let ract = IambAction::from(MessageAction::Redact(reason, desc.bang));
    let step = CommandStep::Continue(ract.into(), ctx.context.clone());

    return Ok(step);
}

fn iamb_reply(desc: CommandDescription, ctx: &mut ProgContext) -> ProgResult {
    if !desc.arg.text.is_empty() {
        return Result::Err(CommandError::InvalidArgument);
    }

    let ract = IambAction::from(MessageAction::Reply);
    let step = CommandStep::Continue(ract.into(), ctx.context.clone());

    return Ok(step);
}

fn iamb_editor(desc: CommandDescription, ctx: &mut ProgContext) -> ProgResult {
    if !desc.arg.text.is_empty() {
        return Result::Err(CommandError::InvalidArgument);
    }

    let sact = IambAction::from(SendAction::SubmitFromEditor);
    let step = CommandStep::Continue(sact.into(), ctx.context.clone());

    return Ok(step);
}

fn iamb_rooms(desc: CommandDescription, ctx: &mut ProgContext) -> ProgResult {
    if !desc.arg.text.is_empty() {
        return Result::Err(CommandError::InvalidArgument);
    }

    let open = ctx.switch(OpenTarget::Application(IambId::RoomList));
    let step = CommandStep::Continue(open, ctx.context.clone());

    return Ok(step);
}

fn iamb_chats(desc: CommandDescription, ctx: &mut ProgContext) -> ProgResult {
    if !desc.arg.text.is_empty() {
        return Result::Err(CommandError::InvalidArgument);
    }

    let open = ctx.switch(OpenTarget::Application(IambId::ChatList));
    let step = CommandStep::Continue(open, ctx.context.clone());

    return Ok(step);
}

fn iamb_unreads(desc: CommandDescription, ctx: &mut ProgContext) -> ProgResult {
    let mut args = desc.arg.strings()?;

    if args.len() > 1 {
        return Result::Err(CommandError::InvalidArgument);
    }

    match args.pop().as_deref() {
        Some("clear") => {
            let clear = IambAction::ClearUnreads;
            let step = CommandStep::Continue(clear.into(), ctx.context.clone());

            return Ok(step);
        },
        Some(_) => return Result::Err(CommandError::InvalidArgument),
        None => {
            let open = ctx.switch(OpenTarget::Application(IambId::UnreadList));
            let step = CommandStep::Continue(open, ctx.context.clone());

            return Ok(step);
        },
    }
}

fn iamb_spaces(desc: CommandDescription, ctx: &mut ProgContext) -> ProgResult {
    if !desc.arg.text.is_empty() {
        return Result::Err(CommandError::InvalidArgument);
    }

    let open = ctx.switch(OpenTarget::Application(IambId::SpaceList));
    let step = CommandStep::Continue(open, ctx.context.clone());

    return Ok(step);
}

fn iamb_welcome(desc: CommandDescription, ctx: &mut ProgContext) -> ProgResult {
    if !desc.arg.text.is_empty() {
        return Result::Err(CommandError::InvalidArgument);
    }

    let open = ctx.switch(OpenTarget::Application(IambId::Welcome));
    let step = CommandStep::Continue(open, ctx.context.clone());

    return Ok(step);
}

fn iamb_join(desc: CommandDescription, ctx: &mut ProgContext) -> ProgResult {
    let mut args = desc.arg.filenames()?;

    if args.len() != 1 {
        return Result::Err(CommandError::InvalidArgument);
    }

    let open = ctx.switch(args.remove(0));
    let step = CommandStep::Continue(open, ctx.context.clone());

    return Ok(step);
}

fn iamb_create(desc: CommandDescription, ctx: &mut ProgContext) -> ProgResult {
    let args = desc.arg.options()?;
    let mut flags = CreateRoomFlags::NONE;
    let mut alias = None;
    let mut ct = CreateRoomType::Room;

    for arg in args {
        match arg {
            OptionType::Flag(name, Some(arg)) => {
                match name.as_str() {
                    "alias" => {
                        if alias.is_some() {
                            let msg = "Multiple ++alias arguments are not allowed";
                            let err = CommandError::Error(msg.into());

                            return Err(err);
                        } else {
                            alias = Some(arg);
                        }
                    },
                    _ => return Err(CommandError::InvalidArgument),
                }
            },
            OptionType::Flag(name, None) => {
                match name.as_str() {
                    "public" => flags |= CreateRoomFlags::PUBLIC,
                    "space" => ct = CreateRoomType::Space,
                    "enc" | "encrypted" => flags |= CreateRoomFlags::ENCRYPTED,
                    _ => return Err(CommandError::InvalidArgument),
                }
            },
            OptionType::Positional(_) => {
                let msg = ":create doesn't take any positional arguments";
                let err = CommandError::Error(msg.into());

                return Err(err);
            },
        }
    }

    let hact = HomeserverAction::CreateRoom(alias, ct, flags);
    let iact = IambAction::from(hact);
    let step = CommandStep::Continue(iact.into(), ctx.context.clone());

    return Ok(step);
}

fn iamb_room(desc: CommandDescription, ctx: &mut ProgContext) -> ProgResult {
    let mut args = desc.arg.strings()?;

    if args.len() < 2 {
        return Result::Err(CommandError::InvalidArgument);
    }

    let field = args.remove(0);
    let action = args.remove(0);

    if args.len() > 1 {
        return Result::Err(CommandError::InvalidArgument);
    }

    let act: IambAction = match (field.as_str(), action.as_str(), args.pop()) {
        // :room dm set
        ("dm", "set", None) => RoomAction::SetDirect(true).into(),
        ("dm", "set", Some(_)) => return Result::Err(CommandError::InvalidArgument),

        // :room dm set
        ("dm", "unset", None) => RoomAction::SetDirect(false).into(),
        ("dm", "unset", Some(_)) => return Result::Err(CommandError::InvalidArgument),

        // :room [kick|ban|unban] <user>
        ("kick", u, r) => {
            RoomAction::MemberUpdate(MemberUpdateAction::Kick, u.into(), r, desc.bang).into()
        },
        ("ban", u, r) => {
            RoomAction::MemberUpdate(MemberUpdateAction::Ban, u.into(), r, desc.bang).into()
        },
        ("unban", u, r) => {
            RoomAction::MemberUpdate(MemberUpdateAction::Unban, u.into(), r, desc.bang).into()
        },

        // :room history set <visibility>
        ("history", "set", Some(s)) => RoomAction::Set(RoomField::History, s).into(),
        ("history", "set", None) => return Result::Err(CommandError::InvalidArgument),

        // :room history unset
        ("history", "unset", None) => RoomAction::Unset(RoomField::History).into(),
        ("history", "unset", Some(_)) => return Result::Err(CommandError::InvalidArgument),

        // :room history show
        ("history", "show", None) => RoomAction::Show(RoomField::History).into(),
        ("history", "show", Some(_)) => return Result::Err(CommandError::InvalidArgument),

        // :room name set <room-name>
        ("name", "set", Some(s)) => RoomAction::Set(RoomField::Name, s).into(),
        ("name", "set", None) => return Result::Err(CommandError::InvalidArgument),

        // :room name unset
        ("name", "unset", None) => RoomAction::Unset(RoomField::Name).into(),
        ("name", "unset", Some(_)) => return Result::Err(CommandError::InvalidArgument),

        // :room topic set <topic>
        ("topic", "set", Some(s)) => RoomAction::Set(RoomField::Topic, s).into(),
        ("topic", "set", None) => return Result::Err(CommandError::InvalidArgument),

        // :room topic unset
        ("topic", "unset", None) => RoomAction::Unset(RoomField::Topic).into(),
        ("topic", "unset", Some(_)) => return Result::Err(CommandError::InvalidArgument),

        // :room topic show
        ("topic", "show", None) => RoomAction::Show(RoomField::Topic).into(),
        ("topic", "show", Some(_)) => return Result::Err(CommandError::InvalidArgument),

        // :room tag set <tag-name>
        ("tag", "set", Some(s)) => RoomAction::Set(RoomField::Tag(tag_name(s)?), "".into()).into(),
        ("tag", "set", None) => return Result::Err(CommandError::InvalidArgument),

        // :room tag unset <tag-name>
        ("tag", "unset", Some(s)) => RoomAction::Unset(RoomField::Tag(tag_name(s)?)).into(),
        ("tag", "unset", None) => return Result::Err(CommandError::InvalidArgument),

        // :room notify set <notification-level>
        ("notify", "set", Some(s)) => RoomAction::Set(RoomField::NotificationMode, s).into(),
        ("notify", "set", None) => return Result::Err(CommandError::InvalidArgument),

        // :room notify unset <notification-level>
        ("notify", "unset", None) => RoomAction::Unset(RoomField::NotificationMode).into(),
        ("notify", "unset", Some(_)) => return Result::Err(CommandError::InvalidArgument),

        // :room notify show
        ("notify", "show", None) => RoomAction::Show(RoomField::NotificationMode).into(),
        ("notify", "show", Some(_)) => return Result::Err(CommandError::InvalidArgument),

        // :room aliases show
        ("alias", "show", None) => RoomAction::Show(RoomField::Aliases).into(),
        ("alias", "show", Some(_)) => return Result::Err(CommandError::InvalidArgument),

        // :room aliases unset <alias>
        ("alias", "unset", Some(s)) => RoomAction::Unset(RoomField::Alias(s)).into(),
        ("alias", "unset", None) => return Result::Err(CommandError::InvalidArgument),

        // :room aliases set <alias>
        ("alias", "set", Some(s)) => RoomAction::Set(RoomField::Alias(s), "".into()).into(),
        ("alias", "set", None) => return Result::Err(CommandError::InvalidArgument),

        // :room canonicalalias show
        ("canonicalalias" | "canon", "show", None) => {
            RoomAction::Show(RoomField::CanonicalAlias).into()
        },
        ("canonicalalias" | "canon", "show", Some(_)) => {
            return Result::Err(CommandError::InvalidArgument)
        },

        // :room canonicalalias set
        ("canonicalalias" | "canon", "set", Some(s)) => {
            RoomAction::Set(RoomField::CanonicalAlias, s).into()
        },
        ("canonicalalias" | "canon", "set", None) => {
            return Result::Err(CommandError::InvalidArgument)
        },

        // :room canonicalalias unset
        ("canonicalalias" | "canon", "unset", None) => {
            RoomAction::Unset(RoomField::CanonicalAlias).into()
        },
        ("canonicalalias" | "canon", "unset", Some(_)) => {
            return Result::Err(CommandError::InvalidArgument)
        },

        // :room id show
        ("id", "show", None) => RoomAction::Show(RoomField::Id).into(),
        ("id", "show", Some(_)) => return Result::Err(CommandError::InvalidArgument),

        _ => return Result::Err(CommandError::InvalidArgument),
    };

    let step = CommandStep::Continue(act.into(), ctx.context.clone());

    return Ok(step);
}

fn iamb_space(desc: CommandDescription, ctx: &mut ProgContext) -> ProgResult {
    let mut args = desc.arg.options()?;

    if args.len() < 2 {
        return Err(CommandError::InvalidArgument);
    }

    let OptionType::Positional(field) = args.remove(0) else {
        return Err(CommandError::InvalidArgument);
    };
    let OptionType::Positional(action) = args.remove(0) else {
        return Err(CommandError::InvalidArgument);
    };

    let act: IambAction = match (field.as_str(), action.as_str()) {
        // :space child remove
        ("child", "remove") => {
            if !(args.is_empty()) {
                return Err(CommandError::InvalidArgument);
            }
            SpaceAction::RemoveChild.into()
        },
        // :space child set <child>
        ("child", "set") => {
            let mut order = None;
            let mut suggested = false;
            let mut raw_child = None;

            for arg in args {
                match arg {
                    OptionType::Flag(name, Some(arg)) => {
                        match name.as_str() {
                            "order" => {
                                if order.is_some() {
                                    let msg = "Multiple ++order arguments are not allowed";
                                    let err = CommandError::Error(msg.into());

                                    return Err(err);
                                } else {
                                    order = Some(arg);
                                }
                            },
                            _ => return Err(CommandError::InvalidArgument),
                        }
                    },
                    OptionType::Flag(name, None) => {
                        match name.as_str() {
                            "suggested" => suggested = true,
                            _ => return Err(CommandError::InvalidArgument),
                        }
                    },
                    OptionType::Positional(arg) => {
                        if raw_child.is_some() {
                            let msg = "Multiple room arguments are not allowed";
                            let err = CommandError::Error(msg.into());

                            return Err(err);
                        }
                        raw_child = Some(arg);
                    },
                }
            }

            let child = if let Some(child) = raw_child {
                OwnedRoomId::from_str(&child)
                    .map_err(|_| CommandError::Error("Invalid room id specified".into()))?
            } else {
                let msg = "Must specify a room to add";
                return Err(CommandError::Error(msg.into()));
            };

            SpaceAction::SetChild(child, order, suggested).into()
        },
        _ => return Result::Err(CommandError::InvalidArgument),
    };

    let step = CommandStep::Continue(act.into(), ctx.context.clone());

    return Ok(step);
}

fn iamb_upload(desc: CommandDescription, ctx: &mut ProgContext) -> ProgResult {
    let mut args = desc.arg.strings()?;

    if args.len() != 1 {
        return Result::Err(CommandError::InvalidArgument);
    }

    let sact = SendAction::Upload(args.remove(0));
    let iact = IambAction::from(sact);
    let step = CommandStep::Continue(iact.into(), ctx.context.clone());

    return Ok(step);
}

fn iamb_tarot(desc: CommandDescription, ctx: &mut ProgContext) -> ProgResult {
    use crate::tarot_cards;
    
    let mut args = desc.arg.strings()?;

    if args.is_empty() {
        let msg = "Usage: :tarot <card-name-or-number> [info] [deepinfo]\nExamples:\n  :tarot fool\n  :tarot fool info\n  :tarot six of swords deepinfo\n  :tarot 3 info\n  :tarot 5 info deepinfo";
        return Result::Err(CommandError::Error(msg.into()));
    }

    // Check for info/deepinfo flags at the end
    let mut show_info = false;
    let mut show_deepinfo = false;
    
    while let Some(last) = args.last() {
        if last == "info" {
            show_info = true;
            args.pop();
        } else if last == "deepinfo" {
            show_deepinfo = true;
            args.pop();
        } else {
            break;
        }
    }

    if args.is_empty() {
        let msg = "No card specified";
        return Result::Err(CommandError::Error(msg.into()));
    }

    // Check if first argument is a number (for N-card spreads)
    if args.len() == 1 {
        if let Ok(num_cards) = args[0].parse::<usize>() {
            if num_cards >= 1 && num_cards <= 10 {
                return handle_n_card_spread(num_cards, show_info, show_deepinfo, ctx);
            } else {
                let msg = "Card count must be between 1 and 10";
                return Err(CommandError::Error(msg.into()));
            }
        }
    }

    // Join all arguments into a single string (for multi-word card names)
    let card_arg = args.join(" ");
    
    // Check for legacy spread types
    if card_arg == "threecard" || card_arg == "three-card" {
        return handle_n_card_spread(3, show_info, show_deepinfo, ctx);
    }
    
    // Check if it's a full path (contains / or starts with ~)
    let is_path = card_arg.contains('/') || card_arg.starts_with('~');
    let file_path = if is_path {
        card_arg.clone()
    } else {
        // Look up card in database
        match tarot_cards::find_card(&card_arg) {
            Some(card) => {
                let path = card.image_path();
                path.to_string_lossy().to_string()
            },
            None => {
                let msg = format!("Card not found: '{}'\nTry: fool, magus, six of swords, science, etc.", card_arg);
                return Err(CommandError::Error(msg));
            }
        }
    };

    // Upload the card image (with text if info requested)
    let sact = if show_info || show_deepinfo {
        // Look up card for info (only if not a direct path)
        let card_lookup = if is_path {
            None
        } else {
            tarot_cards::find_card(&card_arg)
        };
        
        if let Some(card) = card_lookup {
            let info_text = format_card_info(card, show_info, show_deepinfo);
            SendAction::UploadWithText(file_path, info_text)
        } else {
            SendAction::Upload(file_path)
        }
    } else {
        SendAction::Upload(file_path)
    };
    
    let iact = IambAction::from(sact);
    let step = CommandStep::Continue(iact.into(), ctx.context.clone());

    return Ok(step);
}

fn format_card_info(card: &crate::tarot_cards::TarotCard, show_info: bool, show_deepinfo: bool) -> String {
    let mut text = format!("**{}**", card.card);
    
    if let Some(title) = &card.title {
        if !title.is_empty() {
            text.push_str(&format!(" ({})", title));
        }
    }
    text.push_str("\n\n");
    
    if show_info {
        if let Some(info) = &card.info {
            text.push_str(info);
            text.push_str("\n\n");
        }
    }
    
    if show_deepinfo {
        if let Some(deepinfo) = &card.deepinfo {
            text.push_str(deepinfo);
            text.push_str("\n\n");
        }
    }
    
    text
}

fn handle_n_card_spread(num_cards: usize, show_info: bool, show_deepinfo: bool, ctx: &mut ProgContext) -> ProgResult {
    use crate::tarot_cards;
    use crate::tarot_composite;
    use std::collections::HashSet;
    
    // Get all available cards
    let all_cards = tarot_cards::get_all_cards();
    
    if all_cards.len() < num_cards {
        let msg = format!("Not enough tarot cards in database. Need {}, have {}", num_cards, all_cards.len());
        return Err(CommandError::Error(msg));
    }
    
    // Select N random cards
    let mut rng = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let mut selected = HashSet::new();
    while selected.len() < num_cards {
        rng = rng.wrapping_mul(1103515245).wrapping_add(12345);
        let idx = (rng as usize) % all_cards.len();
        selected.insert(idx);
    }
    
    let selected_indices: Vec<usize> = selected.into_iter().collect();
    
    // Get card paths and cards
    let card_paths: Vec<String> = selected_indices
        .iter()
        .map(|&idx| all_cards[idx].image_path().to_string_lossy().to_string())
        .collect();
    
    // Create composite image
    let composite_path = match tarot_composite::save_composite_to_temp(&card_paths) {
        Ok(path) => path,
        Err(e) => {
            let msg = format!("Failed to create composite image: {}", e);
            return Err(CommandError::Error(msg));
        }
    };
    
    // Create info text if requested
    let sact = if show_info || show_deepinfo {
        let mut info_text = String::new();
        for (i, &idx) in selected_indices.iter().enumerate() {
            info_text.push_str(&format!("**Card {}:**\n", i + 1));
            info_text.push_str(&format_card_info(all_cards[idx], show_info, show_deepinfo));
            info_text.push_str("\n");
        }
        SendAction::UploadWithText(composite_path, info_text)
    } else {
        SendAction::Upload(composite_path)
    };
    
    let iact = IambAction::from(sact);
    let step = CommandStep::Continue(iact.into(), ctx.context.clone());

    return Ok(step);
}

fn iamb_tarot_history(desc: CommandDescription, ctx: &mut ProgContext) -> ProgResult {
    use crate::tarot_api;
    
    let args = desc.arg.strings()?;
    
    // TODO: Get user ID from context properly
    // For now, use a placeholder - this will be fixed when integrating with the store
    let matrix_id = "@waqaas:endlessperfect.com".to_string();
    
    if args.is_empty() {
        // Show all readings list
        return show_history_list(&matrix_id, ctx);
    }
    
    match args[0].as_str() {
        "suits" | "suit" => {
            return show_attribute_graph(&matrix_id, "suit", ctx);
        },
        "sephira" => {
            return show_attribute_graph(&matrix_id, "sephira", ctx);
        },
        "planets" | "planet" => {
            return show_attribute_graph(&matrix_id, "planet", ctx);
        },
        "signs" | "sign" => {
            return show_attribute_graph(&matrix_id, "sign", ctx);
        },
        "elements" | "element" => {
            return show_attribute_graph(&matrix_id, "element", ctx);
        },
        "summary" => {
            return show_analytics_summary(&matrix_id, ctx);
        },
        num_str => {
            // Try to parse as reading number
            if let Ok(reading_num) = num_str.parse::<usize>() {
                let show_info = args.len() > 1 && args[1] == "info";
                return show_reading_details(&matrix_id, reading_num, show_info, ctx);
            } else {
                let msg = format!("Invalid argument: '{}'\nUsage: :tarothistory [number|suits|sephira|planets|signs|elements|summary]", num_str);
                return Err(CommandError::Error(msg));
            }
        }
    }
}

fn show_history_list(matrix_id: &str, _ctx: &mut ProgContext) -> ProgResult {
    use crate::tarot_api;
    
    let history = match tarot_api::get_history(matrix_id) {
        Ok(h) => h,
        Err(e) => {
            let msg = format!("Failed to fetch history: {}", e);
            return Err(CommandError::Error(msg));
        }
    };
    
    if history.total_readings == 0 {
        let msg = "No tarot readings found.\nUse :tarot to perform a reading!";
        return Err(CommandError::Error(msg.into()));
    }
    
    let mut output = format!("**Tarot Reading History ({} total readings)**\n\n", history.total_readings);
    
    for (i, reading) in history.readings.iter().enumerate() {
        let card_names: Vec<String> = reading.cards.iter().map(|c| c.card_name.clone()).collect();
        let cards_str = card_names.join(", ");
        
        output.push_str(&format!("{}. {} - {}-card spread\n", 
            i + 1, 
            &reading.reading_date[..10], // Just the date part
            reading.card_count
        ));
        output.push_str(&format!("   {}\n\n", cards_str));
    }
    
    output.push_str("Use :tarothistory <number> to see details\n");
    output.push_str("Use :tarothistory suits/sephira/etc for analytics");
    
    let msg = CommandError::Error(output);
    return Err(msg);
}

fn show_reading_details(matrix_id: &str, reading_num: usize, show_info: bool, _ctx: &mut ProgContext) -> ProgResult {
    use crate::tarot_api;
    
    // First get the list to find the reading_id
    let history = match tarot_api::get_history(matrix_id) {
        Ok(h) => h,
        Err(e) => {
            let msg = format!("Failed to fetch history: {}", e);
            return Err(CommandError::Error(msg));
        }
    };
    
    if reading_num == 0 || reading_num > history.readings.len() {
        let msg = format!("Invalid reading number. Valid range: 1-{}", history.readings.len());
        return Err(CommandError::Error(msg));
    }
    
    let reading_id = history.readings[reading_num - 1].reading_id;
    
    let details = match tarot_api::get_reading_details(reading_id) {
        Ok(d) => d,
        Err(e) => {
            let msg = format!("Failed to fetch reading details: {}", e);
            return Err(CommandError::Error(msg));
        }
    };
    
    let mut output = format!("**Reading #{} - {}**\n", reading_num, &details.reading_date[..10]);
    output.push_str(&format!("Spread: {}\n\n", details.spread_type));
    
    for card in &details.cards {
        let label = card.label.as_ref().map(|l| format!(" ({})", l)).unwrap_or_default();
        output.push_str(&format!("**Card {}{}:** {}\n", card.position + 1, label, card.card_name));
        
        if show_info {
            if let Some(info) = &card.info {
                output.push_str(&format!("{}\n", info));
            }
            if let Some(deepinfo) = &card.deepinfo {
                output.push_str(&format!("{}\n", deepinfo));
            }
        }
        output.push_str("\n");
    }
    
    if let Some(notes) = &details.notes {
        output.push_str(&format!("Notes: {}\n", notes));
    }
    
    let msg = CommandError::Error(output);
    return Err(msg);
}

fn show_attribute_graph(matrix_id: &str, attribute_type: &str, _ctx: &mut ProgContext) -> ProgResult {
    use crate::tarot_api;
    
    let freq = match tarot_api::get_attribute_frequency(matrix_id, attribute_type) {
        Ok(f) => f,
        Err(e) => {
            let msg = format!("Failed to fetch {} frequency: {}", attribute_type, e);
            return Err(CommandError::Error(msg));
        }
    };
    
    if freq.total_count == 0 {
        let msg = format!("No {} data available yet.\nPerform some readings first!", attribute_type);
        return Err(CommandError::Error(msg.into()));
    }
    
    let mut output = format!("**{} Distribution ({} total)**\n\n", 
        attribute_type.to_uppercase(), 
        freq.total_count
    );
    
    let graph = tarot_api::generate_bar_graph(&freq.frequencies, &freq.percentages, 40);
    output.push_str(&graph);
    
    let msg = CommandError::Error(output);
    return Err(msg);
}

fn show_analytics_summary(matrix_id: &str, _ctx: &mut ProgContext) -> ProgResult {
    use crate::tarot_api;
    
    let summary = match tarot_api::get_analytics_summary(matrix_id) {
        Ok(s) => s,
        Err(e) => {
            let msg = format!("Failed to fetch analytics summary: {}", e);
            return Err(CommandError::Error(msg));
        }
    };
    
    let mut output = format!("**Tarot Analytics Summary**\n\n");
    output.push_str(&format!("Total Readings: {}\n", summary.total_readings));
    output.push_str(&format!("Total Cards Drawn: {}\n\n", summary.total_cards_drawn));
    
    output.push_str("**Spread Types:**\n");
    for spread in &summary.spread_types {
        output.push_str(&format!("  {} - {} readings\n", spread.spread_type, spread.count));
    }
    output.push_str("\n");
    
    for (attr_type, top_items) in &summary.top_attributes {
        if !top_items.is_empty() {
            output.push_str(&format!("**Top {}:**\n", attr_type.to_uppercase()));
            for item in top_items {
                output.push_str(&format!("  {} - {}\n", item.value, item.count));
            }
            output.push_str("\n");
        }
    }
    
    let msg = CommandError::Error(output);
    return Err(msg);
}

fn iamb_download(desc: CommandDescription, ctx: &mut ProgContext) -> ProgResult {
    let mut args = desc.arg.strings()?;

    if args.len() > 1 {
        return Result::Err(CommandError::InvalidArgument);
    }

    let mut flags = DownloadFlags::NONE;
    if desc.bang {
        flags |= DownloadFlags::FORCE;
    };
    let mact = MessageAction::Download(args.pop(), flags);
    let iact = IambAction::from(mact);
    let step = CommandStep::Continue(iact.into(), ctx.context.clone());

    return Ok(step);
}

fn iamb_open(desc: CommandDescription, ctx: &mut ProgContext) -> ProgResult {
    let mut args = desc.arg.strings()?;

    if args.len() > 1 {
        return Result::Err(CommandError::InvalidArgument);
    }

    let mut flags = DownloadFlags::OPEN;
    if desc.bang {
        flags |= DownloadFlags::FORCE;
    };
    let mact = MessageAction::Download(args.pop(), flags);
    let iact = IambAction::from(mact);
    let step = CommandStep::Continue(iact.into(), ctx.context.clone());

    return Ok(step);
}

fn iamb_logout(desc: CommandDescription, ctx: &mut ProgContext) -> ProgResult {
    let args = desc.arg.strings()?;

    if args.is_empty() {
        return Result::Err(CommandError::Error("Missing username".to_string()));
    }
    if args.len() != 1 {
        return Result::Err(CommandError::InvalidArgument);
    }

    let iact = IambAction::from(HomeserverAction::Logout(args[0].clone(), desc.bang));
    let step = CommandStep::Continue(iact.into(), ctx.context.clone());

    return Ok(step);
}

fn add_iamb_commands(cmds: &mut ProgramCommands) {
    cmds.add_command(ProgramCommand {
        name: "cancel".into(),
        aliases: vec![],
        f: iamb_cancel,
    });
    cmds.add_command(ProgramCommand {
        name: "create".into(),
        aliases: vec![],
        f: iamb_create,
    });
    cmds.add_command(ProgramCommand {
        name: "chats".into(),
        aliases: vec![],
        f: iamb_chats,
    });
    cmds.add_command(ProgramCommand { name: "dms".into(), aliases: vec![], f: iamb_dms });
    cmds.add_command(ProgramCommand {
        name: "download".into(),
        aliases: vec![],
        f: iamb_download,
    });
    cmds.add_command(ProgramCommand { name: "open".into(), aliases: vec![], f: iamb_open });
    cmds.add_command(ProgramCommand { name: "edit".into(), aliases: vec![], f: iamb_edit });
    cmds.add_command(ProgramCommand {
        name: "invite".into(),
        aliases: vec![],
        f: iamb_invite,
    });
    cmds.add_command(ProgramCommand { name: "join".into(), aliases: vec![], f: iamb_join });
    cmds.add_command(ProgramCommand { name: "keys".into(), aliases: vec![], f: iamb_keys });
    cmds.add_command(ProgramCommand {
        name: "leave".into(),
        aliases: vec![],
        f: iamb_leave,
    });
    cmds.add_command(ProgramCommand {
        name: "members".into(),
        aliases: vec![],
        f: iamb_members,
    });
    cmds.add_command(ProgramCommand {
        name: "react".into(),
        aliases: vec![],
        f: iamb_react,
    });
    cmds.add_command(ProgramCommand {
        name: "redact".into(),
        aliases: vec![],
        f: iamb_redact,
    });
    cmds.add_command(ProgramCommand {
        name: "reply".into(),
        aliases: vec![],
        f: iamb_reply,
    });
    cmds.add_command(ProgramCommand {
        name: "rooms".into(),
        aliases: vec![],
        f: iamb_rooms,
    });
    cmds.add_command(ProgramCommand { name: "room".into(), aliases: vec![], f: iamb_room });
    cmds.add_command(ProgramCommand {
        name: "space".into(),
        aliases: vec![],
        f: iamb_space,
    });
    cmds.add_command(ProgramCommand {
        name: "spaces".into(),
        aliases: vec![],
        f: iamb_spaces,
    });
    cmds.add_command(ProgramCommand {
        name: "unreads".into(),
        aliases: vec![],
        f: iamb_unreads,
    });
    cmds.add_command(ProgramCommand {
        name: "unreact".into(),
        aliases: vec![],
        f: iamb_unreact,
    });
    cmds.add_command(ProgramCommand {
        name: "upload".into(),
        aliases: vec![],
        f: iamb_upload,
    });
    cmds.add_command(ProgramCommand {
        name: "tarot".into(),
        aliases: vec![],
        f: iamb_tarot,
    });
    cmds.add_command(ProgramCommand {
        name: "tarothistory".into(),
        aliases: vec![],
        f: iamb_tarot_history,
    });
    cmds.add_command(ProgramCommand {
        name: "verify".into(),
        aliases: vec![],
        f: iamb_verify,
    });
    cmds.add_command(ProgramCommand {
        name: "welcome".into(),
        aliases: vec![],
        f: iamb_welcome,
    });
    cmds.add_command(ProgramCommand {
        name: "editor".into(),
        aliases: vec![],
        f: iamb_editor,
    });
    cmds.add_command(ProgramCommand {
        name: "logout".into(),
        aliases: vec![],
        f: iamb_logout,
    });
}

/// Initialize the default command state.
pub fn setup_commands() -> ProgramCommands {
    let mut cmds = ProgramCommands::default();

    add_iamb_commands(&mut cmds);

    return cmds;
}

#[cfg(test)]
mod tests {
    use super::*;
    use matrix_sdk::ruma::{room_id, user_id};
    use modalkit::actions::WindowAction;
    use modalkit::editing::context::EditContext;

    #[test]
    fn test_cmd_verify() {
        let mut cmds = setup_commands();
        let ctx = EditContext::default();

        let res = cmds.input_cmd(":verify", ctx.clone()).unwrap();
        let act = WindowAction::Switch(OpenTarget::Application(IambId::VerifyList));
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds.input_cmd(":verify request @user1:example.com", ctx.clone()).unwrap();
        let act = IambAction::VerifyRequest("@user1:example.com".into());
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds
            .input_cmd(":verify accept @user1:example.com/FOOBAR", ctx.clone())
            .unwrap();
        let act = IambAction::Verify(VerifyAction::Accept, "@user1:example.com/FOOBAR".into());
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds
            .input_cmd(":verify mismatch @user2:example.com/QUUXBAZ", ctx.clone())
            .unwrap();
        let act = IambAction::Verify(VerifyAction::Mismatch, "@user2:example.com/QUUXBAZ".into());
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds
            .input_cmd(":verify cancel @user3:example.com/MYDEVICE", ctx.clone())
            .unwrap();
        let act = IambAction::Verify(VerifyAction::Cancel, "@user3:example.com/MYDEVICE".into());
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds
            .input_cmd(":verify confirm @user4:example.com/GOODDEV", ctx.clone())
            .unwrap();
        let act = IambAction::Verify(VerifyAction::Confirm, "@user4:example.com/GOODDEV".into());
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds.input_cmd(":verify confirm", ctx.clone());
        assert_eq!(res, Err(CommandError::InvalidArgument));

        let res = cmds.input_cmd(":verify cancel @user4:example.com MYDEVICE", ctx.clone());
        assert_eq!(res, Err(CommandError::InvalidArgument));

        let res = cmds.input_cmd(":verify mismatch a b c d e f", ctx.clone());
        assert_eq!(res, Err(CommandError::InvalidArgument));
    }

    #[test]
    fn test_cmd_join() {
        let mut cmds = setup_commands();
        let ctx = EditContext::default();

        let res = cmds.input_cmd("join #foobar:example.com", ctx.clone()).unwrap();
        let act = WindowAction::Switch(OpenTarget::Name("#foobar:example.com".into()));
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds.input_cmd("join #", ctx.clone()).unwrap();
        let act = WindowAction::Switch(OpenTarget::Alternate);
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds.input_cmd("join", ctx.clone());
        assert_eq!(res, Err(CommandError::InvalidArgument));

        let res = cmds.input_cmd("join foo bar", ctx.clone());
        assert_eq!(res, Err(CommandError::InvalidArgument));
    }

    #[test]
    fn test_cmd_room_invalid() {
        let mut cmds = setup_commands();
        let ctx = EditContext::default();

        let res = cmds.input_cmd("room", ctx.clone());
        assert_eq!(res, Err(CommandError::InvalidArgument));

        let res = cmds.input_cmd("room foo", ctx.clone());
        assert_eq!(res, Err(CommandError::InvalidArgument));

        let res = cmds.input_cmd("room set topic", ctx.clone());
        assert_eq!(res, Err(CommandError::InvalidArgument));
    }

    #[test]
    fn test_cmd_room_topic_set() {
        let mut cmds = setup_commands();
        let ctx = EditContext::default();

        let res = cmds
            .input_cmd("room topic set \"Lots of fun discussion!\"", ctx.clone())
            .unwrap();
        let act = RoomAction::Set(RoomField::Topic, "Lots of fun discussion!".into());
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds
            .input_cmd("room topic set The\\ Discussion\\ Room", ctx.clone())
            .unwrap();
        let act = RoomAction::Set(RoomField::Topic, "The Discussion Room".into());
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds.input_cmd("room topic set Development", ctx.clone()).unwrap();
        let act = RoomAction::Set(RoomField::Topic, "Development".into());
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds.input_cmd("room topic", ctx.clone());
        assert_eq!(res, Err(CommandError::InvalidArgument));

        let res = cmds.input_cmd("room topic set", ctx.clone());
        assert_eq!(res, Err(CommandError::InvalidArgument));

        let res = cmds.input_cmd("room topic set A B C", ctx.clone());
        assert_eq!(res, Err(CommandError::InvalidArgument));
    }

    #[test]
    fn test_cmd_room_name_invalid() {
        let mut cmds = setup_commands();
        let ctx = EditContext::default();

        let res = cmds.input_cmd("room name", ctx.clone());
        assert_eq!(res, Err(CommandError::InvalidArgument));

        let res = cmds.input_cmd("room name foo", ctx.clone());
        assert_eq!(res, Err(CommandError::InvalidArgument));
    }

    #[test]
    fn test_cmd_room_name_set() {
        let mut cmds = setup_commands();
        let ctx = EditContext::default();

        let res = cmds.input_cmd("room name set Development", ctx.clone()).unwrap();
        let act = RoomAction::Set(RoomField::Name, "Development".into());
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds
            .input_cmd("room name set \"Application Development\"", ctx.clone())
            .unwrap();
        let act = RoomAction::Set(RoomField::Name, "Application Development".into());
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds.input_cmd("room name set", ctx.clone());
        assert_eq!(res, Err(CommandError::InvalidArgument));
    }

    #[test]
    fn test_cmd_room_name_unset() {
        let mut cmds = setup_commands();
        let ctx = EditContext::default();

        let res = cmds.input_cmd("room name unset", ctx.clone()).unwrap();
        let act = RoomAction::Unset(RoomField::Name);
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds.input_cmd("room name unset foo", ctx.clone());
        assert_eq!(res, Err(CommandError::InvalidArgument));
    }

    #[test]
    fn test_cmd_room_dm_set() {
        let mut cmds = setup_commands();
        let ctx = EditContext::default();

        let res = cmds.input_cmd("room dm set", ctx.clone()).unwrap();
        let act = RoomAction::SetDirect(true);
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds.input_cmd("room dm set true", ctx.clone());
        assert_eq!(res, Err(CommandError::InvalidArgument));
    }

    #[test]
    fn test_cmd_room_dm_unset() {
        let mut cmds = setup_commands();
        let ctx = EditContext::default();

        let res = cmds.input_cmd("room dm unset", ctx.clone()).unwrap();
        let act = RoomAction::SetDirect(false);
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds.input_cmd("room dm unset true", ctx.clone());
        assert_eq!(res, Err(CommandError::InvalidArgument));
    }

    #[test]
    fn test_cmd_room_tag_set() {
        let mut cmds = setup_commands();
        let ctx = EditContext::default();

        let res = cmds.input_cmd("room tag set favourite", ctx.clone()).unwrap();
        let act = RoomAction::Set(RoomField::Tag(TagName::Favorite), "".into());
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds.input_cmd("room tag set favorite", ctx.clone()).unwrap();
        let act = RoomAction::Set(RoomField::Tag(TagName::Favorite), "".into());
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds.input_cmd("room tag set fav", ctx.clone()).unwrap();
        let act = RoomAction::Set(RoomField::Tag(TagName::Favorite), "".into());
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds.input_cmd("room tag set low_priority", ctx.clone()).unwrap();
        let act = RoomAction::Set(RoomField::Tag(TagName::LowPriority), "".into());
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds.input_cmd("room tag set low-priority", ctx.clone()).unwrap();
        let act = RoomAction::Set(RoomField::Tag(TagName::LowPriority), "".into());
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds.input_cmd("room tag set low", ctx.clone()).unwrap();
        let act = RoomAction::Set(RoomField::Tag(TagName::LowPriority), "".into());
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds.input_cmd("room tag set servernotice", ctx.clone()).unwrap();
        let act = RoomAction::Set(RoomField::Tag(TagName::ServerNotice), "".into());
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds.input_cmd("room tag set server_notice", ctx.clone()).unwrap();
        let act = RoomAction::Set(RoomField::Tag(TagName::ServerNotice), "".into());
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds.input_cmd("room tag set server_notice", ctx.clone()).unwrap();
        let act = RoomAction::Set(RoomField::Tag(TagName::ServerNotice), "".into());
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds.input_cmd("room tag set u.custom-tag", ctx.clone()).unwrap();
        let act = RoomAction::Set(
            RoomField::Tag(TagName::User("u.custom-tag".parse().unwrap())),
            "".into(),
        );
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds.input_cmd("room tag set u.irc", ctx.clone()).unwrap();
        let act =
            RoomAction::Set(RoomField::Tag(TagName::User("u.irc".parse().unwrap())), "".into());
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds.input_cmd("room tag", ctx.clone());
        assert_eq!(res, Err(CommandError::InvalidArgument));

        let res = cmds.input_cmd("room tag set", ctx.clone());
        assert_eq!(res, Err(CommandError::InvalidArgument));

        let res = cmds.input_cmd("room tag set unknown", ctx.clone());
        assert_eq!(res, Err(CommandError::Error("Invalid user tag name: unknown".into())));

        let res = cmds.input_cmd("room tag set needs-leading-u-dot", ctx.clone());
        assert_eq!(
            res,
            Err(CommandError::Error("Invalid user tag name: needs-leading-u-dot".into()))
        );
    }

    #[test]
    fn test_cmd_room_tag_unset() {
        let mut cmds = setup_commands();
        let ctx = EditContext::default();

        let res = cmds.input_cmd("room tag unset favourite", ctx.clone()).unwrap();
        let act = RoomAction::Unset(RoomField::Tag(TagName::Favorite));
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds.input_cmd("room tag unset favorite", ctx.clone()).unwrap();
        let act = RoomAction::Unset(RoomField::Tag(TagName::Favorite));
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds.input_cmd("room tag unset fav", ctx.clone()).unwrap();
        let act = RoomAction::Unset(RoomField::Tag(TagName::Favorite));
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds.input_cmd("room tag unset low_priority", ctx.clone()).unwrap();
        let act = RoomAction::Unset(RoomField::Tag(TagName::LowPriority));
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds.input_cmd("room tag unset low-priority", ctx.clone()).unwrap();
        let act = RoomAction::Unset(RoomField::Tag(TagName::LowPriority));
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds.input_cmd("room tag unset low", ctx.clone()).unwrap();
        let act = RoomAction::Unset(RoomField::Tag(TagName::LowPriority));
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds.input_cmd("room tag unset servernotice", ctx.clone()).unwrap();
        let act = RoomAction::Unset(RoomField::Tag(TagName::ServerNotice));
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds.input_cmd("room tag unset server_notice", ctx.clone()).unwrap();
        let act = RoomAction::Unset(RoomField::Tag(TagName::ServerNotice));
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds.input_cmd("room tag unset server_notice", ctx.clone()).unwrap();
        let act = RoomAction::Unset(RoomField::Tag(TagName::ServerNotice));
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds.input_cmd("room tag unset u.custom-tag", ctx.clone()).unwrap();
        let act = RoomAction::Unset(RoomField::Tag(TagName::User("u.custom-tag".parse().unwrap())));
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds.input_cmd("room tag unset u.irc", ctx.clone()).unwrap();
        let act = RoomAction::Unset(RoomField::Tag(TagName::User("u.irc".parse().unwrap())));
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds.input_cmd("room tag", ctx.clone());
        assert_eq!(res, Err(CommandError::InvalidArgument));

        let res = cmds.input_cmd("room tag set", ctx.clone());
        assert_eq!(res, Err(CommandError::InvalidArgument));

        let res = cmds.input_cmd("room tag unset unknown", ctx.clone());
        assert_eq!(res, Err(CommandError::Error("Invalid user tag name: unknown".into())));

        let res = cmds.input_cmd("room tag unset needs-leading-u-dot", ctx.clone());
        assert_eq!(
            res,
            Err(CommandError::Error("Invalid user tag name: needs-leading-u-dot".into()))
        );
    }

    #[test]
    fn test_cmd_room_notification_mode_set() {
        let mut cmds = setup_commands();
        let ctx = EditContext::default();

        let cmd = "room notify set mute";
        let res = cmds.input_cmd(cmd, ctx.clone()).unwrap();
        let act = RoomAction::Set(RoomField::NotificationMode, "mute".into());
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let cmd = "room notify unset";
        let res = cmds.input_cmd(cmd, ctx.clone()).unwrap();
        let act = RoomAction::Unset(RoomField::NotificationMode);
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let cmd = "room notify show";
        let res = cmds.input_cmd(cmd, ctx.clone()).unwrap();
        let act = RoomAction::Show(RoomField::NotificationMode);
        assert_eq!(res, vec![(act.into(), ctx.clone())]);
    }

    #[test]
    fn test_cmd_room_id_show() {
        let mut cmds = setup_commands();
        let ctx = EditContext::default();

        let res = cmds.input_cmd("room id show", ctx.clone()).unwrap();
        let act = RoomAction::Show(RoomField::Id);
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds.input_cmd("room id show foo", ctx.clone());
        assert_eq!(res, Err(CommandError::InvalidArgument));
    }

    #[test]
    fn test_cmd_space_child() {
        let mut cmds = setup_commands();
        let ctx = EditContext::default();

        let cmd = "space";
        let res = cmds.input_cmd(cmd, ctx.clone());
        assert_eq!(res, Err(CommandError::InvalidArgument));

        let cmd = "space ++foo bar baz";
        let res = cmds.input_cmd(cmd, ctx.clone());
        assert_eq!(res, Err(CommandError::InvalidArgument));

        let cmd = "space child foo";
        let res = cmds.input_cmd(cmd, ctx.clone());
        assert_eq!(res, Err(CommandError::InvalidArgument));
    }

    #[test]
    fn test_cmd_space_child_set() {
        let mut cmds = setup_commands();
        let ctx = EditContext::default();

        let cmd = "space child set !roomid:example.org";
        let res = cmds.input_cmd(cmd, ctx.clone()).unwrap();
        let act = SpaceAction::SetChild(room_id!("!roomid:example.org").to_owned(), None, false);
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let cmd = "space child set ++order=abcd ++suggested !roomid:example.org";
        let res = cmds.input_cmd(cmd, ctx.clone()).unwrap();
        let act = SpaceAction::SetChild(
            room_id!("!roomid:example.org").to_owned(),
            Some("abcd".into()),
            true,
        );
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let cmd = "space child set ++order=abcd ++order=1234 !roomid:example.org";
        let res = cmds.input_cmd(cmd, ctx.clone());
        assert_eq!(
            res,
            Err(CommandError::Error("Multiple ++order arguments are not allowed".into()))
        );

        let cmd = "space child set !roomid:example.org !otherroom:example.org";
        let res = cmds.input_cmd(cmd, ctx.clone());
        assert_eq!(res, Err(CommandError::Error("Multiple room arguments are not allowed".into())));

        let cmd = "space child set ++foo=abcd !roomid:example.org";
        let res = cmds.input_cmd(cmd, ctx.clone());
        assert_eq!(res, Err(CommandError::InvalidArgument));

        let cmd = "space child set ++foo !roomid:example.org";
        let res = cmds.input_cmd(cmd, ctx.clone());
        assert_eq!(res, Err(CommandError::InvalidArgument));

        let cmd = "space child ++order=abcd ++suggested set !roomid:example.org";
        let res = cmds.input_cmd(cmd, ctx.clone());
        assert_eq!(res, Err(CommandError::InvalidArgument));

        let cmd = "space child set foo";
        let res = cmds.input_cmd(cmd, ctx.clone());
        assert_eq!(res, Err(CommandError::Error("Invalid room id specified".into())));

        let cmd = "space child set";
        let res = cmds.input_cmd(cmd, ctx.clone());
        assert_eq!(res, Err(CommandError::Error("Must specify a room to add".into())));
    }

    #[test]
    fn test_cmd_space_child_remove() {
        let mut cmds = setup_commands();
        let ctx = EditContext::default();

        let cmd = "space child remove";
        let res = cmds.input_cmd(cmd, ctx.clone()).unwrap();
        let act = SpaceAction::RemoveChild;
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let cmd = "space child remove foo";
        let res = cmds.input_cmd(cmd, ctx.clone());
        assert_eq!(res, Err(CommandError::InvalidArgument));
    }

    #[test]
    fn test_cmd_invite() {
        let mut cmds = setup_commands();
        let ctx = EditContext::default();

        let res = cmds.input_cmd("invite accept", ctx.clone()).unwrap();
        let act = IambAction::Room(RoomAction::InviteAccept);
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds.input_cmd("invite reject", ctx.clone()).unwrap();
        let act = IambAction::Room(RoomAction::InviteReject);
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds.input_cmd("invite send @user:example.com", ctx.clone()).unwrap();
        let act =
            IambAction::Room(RoomAction::InviteSend(user_id!("@user:example.com").to_owned()));
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds.input_cmd("invite", ctx.clone());
        assert_eq!(res, Err(CommandError::InvalidArgument));

        let res = cmds.input_cmd("invite foo", ctx.clone());
        assert_eq!(res, Err(CommandError::InvalidArgument));

        let res = cmds.input_cmd("invite accept @user:example.com", ctx.clone());
        assert_eq!(res, Err(CommandError::InvalidArgument));

        let res = cmds.input_cmd("invite reject @user:example.com", ctx.clone());
        assert_eq!(res, Err(CommandError::InvalidArgument));

        let res = cmds.input_cmd("invite send", ctx.clone());
        assert_eq!(res, Err(CommandError::InvalidArgument));

        let res = cmds.input_cmd("invite @user:example.com", ctx.clone());
        assert_eq!(res, Err(CommandError::InvalidArgument));
    }

    #[test]
    fn test_cmd_room_kick() {
        let mut cmds = setup_commands();
        let ctx = EditContext::default();

        let res = cmds.input_cmd("room kick @user:example.com", ctx.clone()).unwrap();
        let act = IambAction::Room(RoomAction::MemberUpdate(
            MemberUpdateAction::Kick,
            "@user:example.com".into(),
            None,
            false,
        ));
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds.input_cmd("room! kick @user:example.com", ctx.clone()).unwrap();
        let act = IambAction::Room(RoomAction::MemberUpdate(
            MemberUpdateAction::Kick,
            "@user:example.com".into(),
            None,
            true,
        ));
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds
            .input_cmd("room! kick @user:example.com \"reason here\"", ctx.clone())
            .unwrap();
        let act = IambAction::Room(RoomAction::MemberUpdate(
            MemberUpdateAction::Kick,
            "@user:example.com".into(),
            Some("reason here".into()),
            true,
        ));
        assert_eq!(res, vec![(act.into(), ctx.clone())]);
    }

    #[test]
    fn test_cmd_room_ban_unban() {
        let mut cmds = setup_commands();
        let ctx = EditContext::default();

        let res = cmds
            .input_cmd("room! ban @user:example.com \"spam\"", ctx.clone())
            .unwrap();
        let act = IambAction::Room(RoomAction::MemberUpdate(
            MemberUpdateAction::Ban,
            "@user:example.com".into(),
            Some("spam".into()),
            true,
        ));
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds
            .input_cmd("room unban @user:example.com \"reconciled\"", ctx.clone())
            .unwrap();
        let act = IambAction::Room(RoomAction::MemberUpdate(
            MemberUpdateAction::Unban,
            "@user:example.com".into(),
            Some("reconciled".into()),
            false,
        ));
        assert_eq!(res, vec![(act.into(), ctx.clone())]);
    }

    #[test]
    fn test_cmd_redact() {
        let mut cmds = setup_commands();
        let ctx = EditContext::default();

        let res = cmds.input_cmd("redact", ctx.clone()).unwrap();
        let act = IambAction::Message(MessageAction::Redact(None, false));
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds.input_cmd("redact!", ctx.clone()).unwrap();
        let act = IambAction::Message(MessageAction::Redact(None, true));
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds.input_cmd("redact Removed", ctx.clone()).unwrap();
        let act = IambAction::Message(MessageAction::Redact(Some("Removed".into()), false));
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds.input_cmd("redact \"Removed\"", ctx.clone()).unwrap();
        let act = IambAction::Message(MessageAction::Redact(Some("Removed".into()), false));
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds.input_cmd("redact Removed Removed", ctx.clone());
        assert_eq!(res, Err(CommandError::InvalidArgument));
    }

    #[test]
    fn test_cmd_keys() {
        let mut cmds = setup_commands();
        let ctx = EditContext::default();

        let res = cmds.input_cmd("keys import /a/b/c pword", ctx.clone()).unwrap();
        let act = IambAction::Keys(KeysAction::Import("/a/b/c".into(), "pword".into()));
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        let res = cmds.input_cmd("keys export /a/b/c pword", ctx.clone()).unwrap();
        let act = IambAction::Keys(KeysAction::Export("/a/b/c".into(), "pword".into()));
        assert_eq!(res, vec![(act.into(), ctx.clone())]);

        // Invalid invocations.
        let res = cmds.input_cmd("keys", ctx.clone());
        assert_eq!(res, Err(CommandError::InvalidArgument));

        let res = cmds.input_cmd("keys import", ctx.clone());
        assert_eq!(res, Err(CommandError::InvalidArgument));

        let res = cmds.input_cmd("keys import foo", ctx.clone());
        assert_eq!(res, Err(CommandError::InvalidArgument));

        let res = cmds.input_cmd("keys import foo bar baz", ctx.clone());
        assert_eq!(res, Err(CommandError::InvalidArgument));
    }
}
