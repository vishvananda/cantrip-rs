// Old-Growth Troll
// status: planned
// requires: granted Static
// Trample
// When Old-Growth Troll dies, if it was a creature, return it to the battlefield. It's an Aura enchantment with enchant Forest you control and "Enchanted Forest has '{T}: Add {G}{G}' and '{1}, {T}, Sacrifice this land: Create a tapped 4/4 green Troll Warrior creature token with trample.'"
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *old; let subject = Target::Object(id); let data = [new.0, u32::MAX]; if to.is(ZoneKind::Graveyard) && from.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) && subject.object().is_some_and(|candidate: ObjectId| q.chars(candidate).is(CardType::Creature)) { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn becomes_aura_2(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Forest)) && Some(asker.controller) == Some(q.controller(id))))) }
fn granted_4_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_5 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_5_0 = { let q: &dyn Query = g; (2i64).max(0) as u32 };
for player in mana_players_5 {
g.add_mana(player, Mana::G, mana_5_0);}
 Ok(()) }
pub mod token_8 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Troll Warrior).name, token_words: token!(Troll Warrior).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Troll,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(16)), power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn granted_6(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_7 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_7 = Vec::new();
let count_7_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_7_0 = g.card_id(&token_8::CARD);
for player in players_7 {
tokens_7.extend(std::iter::repeat((def_7_0,player)).take(count_7_0 as usize));
} let words_7 = g.words(asker.source);
let created_7 = g.create_tokens_worded(&tokens_7, true, None, words_7)?;
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn granted_3_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let card = { let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }; let _ = card; if let Some(slot) = chars.abilities.iter().position(|a| a.is_none()) { chars.abilities[slot] = Some(AbilityRef::delayed({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0)); }
let card = { let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }; let _ = card; if let Some(slot) = chars.abilities.iter().position(|a| a.is_none()) { chars.abilities[slot] = Some(AbilityRef::delayed({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 1)); } }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller); if !{ let q: &dyn Query = g; item.subject.object().is_some_and(|candidate: ObjectId| q.chars(candidate).is(CardType::Creature)) } { return Ok(()); } let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_0 = ({ let q: &dyn Query = g; ((Some(ObjectId(item.data[0])).filter(|o| *o != ObjectId::NONE).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Graveyard).collect::<Vec<ObjectId>>()).get(0).copied().into_iter().collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_0 = { let rows_0: Vec<(ObjectId, PlayerId, bool)> = objects_0.iter().map(|&o| (o, player_0, false)).collect(); let came_0 = g.put_all_onto_battlefield_as(&rows_0)?; came_0.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
let objects_1 = { let q: &dyn Query = g; ((moved_0.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_1 = { let q: &dyn Query = g; Some(asker.source) }; let duration_1 = Duration::Indefinite;
let kind_1_0_0 = { let q: &dyn Query = g; EffectKind::SetTypes(TypeSet::of(&[CardType::Enchantment])) };
let kind_1_1_0 = { let q: &dyn Query = g; EffectKind::AddSubtype(Subtype::Aura) };
let kind_1_2_0 = { let q: &dyn Query = g; EffectKind::Enchant({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }) };
let kind_1_3_0 = { let q: &dyn Query = g; EffectKind::AddAbility(planned_value!("granted Static", AbilityRef::delayed({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0) => AbilityRef::granted_static({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0))) };
if let Some(source) = source_1 { cantrip_api::instruction(g, |g| { for &object in &objects_1 { g.add_effect(source, object, kind_1_0_0, duration_1); g.add_effect(source, object, kind_1_1_0, duration_1); g.add_effect(source, object, kind_1_2_0, duration_1); g.add_effect(source, object, kind_1_3_0, duration_1);  } }); }
g.end_sentence();
 Ok(()) }
planned_item!("granted-static" => granted_rules: CardDef { keywords: Keywords::of(&[]), abilities: &[], statics: &[static_ability(Layer::Ability, cantrip_mechanics::attach::attached_object, granted_3_apply_0, "When Old-Growth Troll dies, if it was a creature, return it to the battlefield. It's an Aura enchantment with enchant Forest you control and \"Enchanted Forest has '{T}: Add {G}{G}' and '{1}, {T}, Sacrifice this land: Create a tapped 4/4 green Troll Warrior creature token with trample.'\"")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], enters: None, ..CardDef::BLANK });

pub static CARD: CardDef = CardDef { name: "Old-Growth Troll", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 3, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Troll,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "When Old-Growth Troll dies, if it was a creature, return it to the battlefield. It's an Aura enchantment with enchant Forest you control and \"Enchanted Forest has '{T}: Add {G}{G}' and '{1}, {T}, Sacrifice this land: Create a tapped 4/4 green Troll Warrior creature token with trample.'\"").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), Adds { pool: ManaPool([0, 0, 0, 0, 2, 0]), ..add_by(granted_4_mana) }, "Enchanted Forest has '{T}: Add {G}{G}' and '{1}, {T}, Sacrifice this land: Create a tapped 4/4 green Troll Warrior creature token with trample.'"),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(SACRIFICE_THIS).and(Cost { tap: true, ..Cost::NONE }), granted_6, "Enchanted Forest has '{T}: Add {G}{G}' and '{1}, {T}, Sacrifice this land: Create a tapped 4/4 green Troll Warrior creature token with trample.'").targets(&[])], enters: None, modal: None, enchant: None, becomes_aura_enchant: Some(target(becomes_aura_2)), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_8::CARD];
