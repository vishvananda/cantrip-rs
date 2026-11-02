// Animate Dead
// status: planned
// requires: an Aura that enchants what it put onto the battlefield
// requires: enchant graveyard card
// requires: reflexive ability about two objects
// Enchant creature card in a graveyard
// When this Aura enters, if it's on the battlefield, it loses "enchant creature card in a graveyard" and gains "enchant creature put onto the battlefield with this Aura." Return enchanted creature card to the battlefield under your control and attach this Aura to it. When this Aura leaves the battlefield, that creature's controller sacrifices it.
// Enchanted creature gets -1/-0.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_1_if(q: &dyn Query, asker: &Asker) -> bool { Some(asker.source).is_some_and(|candidate: ObjectId| (q.state().obj(candidate).became.is_none() && q.state().obj(candidate).zone.kind() == ZoneKind::Battlefield)) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn delayed_4_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, .. } = event { let id = *old; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if from.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn delayed_4(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("reflexive-ability-about-two-objects", Sacrifice, "choose" = "()", "objects" = { let q: &dyn Query = g; item.subject.object().into_iter().collect::<Vec<ObjectId>>() }, "players" = "{\"args\":{\"args\":{\"kind\":\"EventSubject\"},\"kind\":\"ControllerOf\"},\"kind\":\"One\"}");
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("an-aura-that-enchants-what-it-put-onto-the-battlefield", CreateContinuousEffect, "affected" = "{\"Objects\":{\"args\":{\"kind\":\"Source\"},\"kind\":\"One\"}}", "changes" = "[{\"args\":{\"args\":{\"family\":\"CheckHasable\",\"form\":\"Enchant\",\"operands\":[{\"args\":{\"args\":{\"args\":\"Creature\",\"kind\":\"CardType\"},\"kind\":\"Battlefield\"},\"kind\":\"Objects\"}]},\"kind\":\"Term\"},\"kind\":\"LosesAbility\"},{\"args\":{\"args\":{\"hosts\":{\"Permanents\":{\"args\":[{\"args\":\"Creature\",\"kind\":\"CardType\"},{\"args\":{\"kind\":\"Source\"},\"kind\":\"MadeBy\"}],\"kind\":\"All\"}}},\"kind\":\"Enchant\"},\"kind\":\"GrantAbility\"}]", "duration" = "\"Indefinite\"", "evaluation" = "\"Snapshot\"", "source" = { let q: &dyn Query = g; Some(asker.source) });
g.end_sentence();
let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_1 = ({ let q: &dyn Query = g; Some(asker.source).and_then(|o: ObjectId| q.state().obj(o).attached_to).into_iter().collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_1 = { let rows_1: Vec<(ObjectId, PlayerId, bool)> = objects_1.iter().map(|&o| (o, player_1, false)).collect(); let came_1 = g.put_all_onto_battlefield_as(&rows_1)?; came_1.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
let attachments_2 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let host_2 = { let q: &dyn Query = g; (moved_1.clone()).get(0 as usize).copied() }; if let Some(host) = host_2 { cantrip_api::instruction(g, |g| { for attachment in attachments_2 { g.attach(attachment, host); } }); }
g.end_sentence();
if let Some(source) = Some(item.object) { g.delayed_trigger(DelayedWhen::UntilEvent, source, item.controller, AbilityRef::delayed(item.ability.card, 0), { let q: &dyn Query = g; (moved_1.clone()).get(0 as usize).copied() }.map_or(Target::None, Target::Object)); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = ((-1i64) as i16, (0i64) as i16); chars.power = chars.power.map(|v| v.saturating_add(p)); chars.toughness = chars.toughness.map(|v| v.saturating_add(t)); }
planned_item!("enchant-graveyard-card" => enchant: Some(cantrip_mechanics::targets::graveyard_card(|q: &dyn Query, asker: &Asker, owner: PlayerId| true, |q: &dyn Query, asker: &Asker, id: ObjectId| -> bool { q.chars(id).is(CardType::Creature) })));

pub static CARD: CardDef = CardDef { name: "Animate Dead", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Aura]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "When this Aura enters, if it's on the battlefield, it loses \"enchant creature card in a graveyard\" and gains \"enchant creature put onto the battlefield with this Aura.\" Return enchanted creature card to the battlefield under your control and attach this Aura to it. When this Aura leaves the battlefield, that creature's controller sacrifices it.").targets(&[]).condition(ability_1_if)], statics: &[static_ability(Layer::PtModify, cantrip_mechanics::attach::attached_object, ability_2_apply_0, "Enchanted creature gets -1/-0.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), delayed_4_event), delayed_4, "When this Aura leaves the battlefield, that creature's controller sacrifices it.").targets(&[])], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
