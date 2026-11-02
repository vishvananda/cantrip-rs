// Calamity's Wake
// status: planned
// requires: player rule for a while can't cast some spells
// Exile all graveyards. Players can't cast noncreature spells this turn. Exile Calamity's Wake.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("player-rule-for-a-while-can-t-cast-some-spells" => #[allow(unused_variables)] fn lasting_spell_rule_0(q: &dyn Query, asker: &Asker, about: Target, player: PlayerId, spell: ObjectId) -> bool { let source = asker.source; !(q.chars(spell).is(CardType::Creature)) });
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; ((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() }; let moved_0 = g.exile_all(&objects_0);
g.end_sentence();
planned!("player-rule-for-a-while-can-t-cast-some-spells" => let players_1_0 = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }; if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { for &player in &players_1_0 { g.add_player_effect(asker.controller, source, EffectKind::CantCast { player, card: item.ability.card, index: 0, about: Target::None }, Duration::EndOfTurn); } });
g.end_sentence();
let objects_2 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_2 = g.exile_all(&objects_2);
g.end_sentence();
 Ok(()) }
planned_item!("player-rule-for-a-while-can-t-cast-some-spells" => lasting_spell_rules: lasting_spell_rule_0);

pub static CARD: CardDef = CardDef { name: "Calamity's Wake", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
