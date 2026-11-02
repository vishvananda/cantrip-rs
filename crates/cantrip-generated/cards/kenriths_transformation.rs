// Kenrith's Transformation
// status: ready
// Enchant creature
// When this Aura enters, draw a card.
// Enchanted creature loses all abilities and is a green Elk creature with base power and toughness 3/3. (It loses all other card types and creature types.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_enchant(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.draw(player, amount_0); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_apply_3(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let kept = TypeSet::of(&[CardType::Creature]); chars.types = kept; chars.subtypes = chars.subtypes.iter().filter(|s| s.card_types().0 & kept.0 != 0).fold(SubtypeSet::NONE, SubtypeSet::with);
chars.subtypes = chars.subtypes.minus(SubtypeSet::CREATURE_TYPES); if (chars.is(CardType::Creature) || chars.is(CardType::Kindred)) { chars.subtypes = chars.subtypes.with(q.words(asker.source).subtype(Subtype::Elk)); } }
#[allow(unused_variables)] fn ability_2_apply_1(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.colors = ColorSet::NONE.with(q.words(asker.source).color(Color::Green)); }
#[allow(unused_variables)] fn ability_2_apply_4(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = ((3i64) as i16, (3i64) as i16); chars.power = Some(p); chars.toughness = Some(t); }

pub static CARD: CardDef = CardDef { name: "Kenrith's Transformation", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Aura]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "When this Aura enters, draw a card.").targets(&[])], statics: &[static_ability(Layer::Type, cantrip_mechanics::attach::attached_object, ability_2_apply_3, "Enchanted creature loses all abilities and is a green Elk creature with base power and toughness 3/3."),static_ability(Layer::Color, cantrip_mechanics::attach::attached_object, ability_2_apply_1, "Enchanted creature loses all abilities and is a green Elk creature with base power and toughness 3/3.").continues(),loses_all_abilities(cantrip_mechanics::attach::attached_object, "Enchanted creature loses all abilities and is a green Elk creature with base power and toughness 3/3.").continues(),static_ability(Layer::PtSet, cantrip_mechanics::attach::attached_object, ability_2_apply_4, "Enchanted creature loses all abilities and is a green Elk creature with base power and toughness 3/3.").continues()], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: Some(target(ability_0_enchant)), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
