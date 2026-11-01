// Cloudform
// status: ready
// When this enchantment enters, it becomes an Aura with enchant creature. Manifest the top card of your library and attach this enchantment to it. (To manifest a card, put it onto the battlefield face down as a 2/2 creature. Turn it face up any time for its mana cost if it's a creature card.)
// Enchanted creature has flying and hexproof.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn becomes_aura_1(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_0 = { let q: &dyn Query = g; Some(asker.source) }; let duration_0 = Duration::Indefinite;
let kind_0_0_0 = { let q: &dyn Query = g; EffectKind::AddSubtype(Subtype::Aura) };
let kind_0_1_0 = { let q: &dyn Query = g; EffectKind::Enchant({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }) };
if let Some(source) = source_0 { cantrip_api::instruction(g, |g| { for &object in &objects_0 { g.add_effect(source, object, kind_0_0_0, duration_0); g.add_effect(source, object, kind_0_1_0, duration_0);  } }); }
g.end_sentence();
let controller_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let owner_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let count_2 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let mut entered_2: Vec<ObjectId> = Vec::new(); for _ in 0..count_2 { let Some(&card) = cantrip_mechanics::top_of_library(g, owner_2, 1).first() else { break; }; entered_2.extend(g.put_onto_battlefield_face_down(card, controller_2, FaceDown::Manifested, false)?); }
g.end_sentence();
let attachments_3 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let host_3 = { let q: &dyn Query = g; (entered_2.clone()).get(0 as usize).copied() }; if let Some(host) = host_3 { cantrip_api::instruction(g, |g| { for attachment in attachments_3 { g.attach(attachment, host); } }); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::Flying); }
#[allow(unused_variables)] fn ability_1_apply_1(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::Hexproof); }

pub static CARD: CardDef = CardDef { name: "Cloudform", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When this enchantment enters, it becomes an Aura with enchant creature. Manifest the top card of your library and attach this enchantment to it.").targets(&[])], statics: &[static_ability(Layer::Ability, cantrip_mechanics::attach::attached_object, ability_1_apply_0, "Enchanted creature has flying and hexproof."),static_ability(Layer::Ability, cantrip_mechanics::attach::attached_object, ability_1_apply_1, "Enchanted creature has flying and hexproof.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, becomes_aura_enchant: Some(target(becomes_aura_1)), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
