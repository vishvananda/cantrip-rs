// Experimental Frenzy
// status: ready
// You may look at the top card of your library any time.
// You may play lands and cast spells from the top of your library.
// You can't play lands or cast spells from your hand.
// {3}{R}: Destroy this enchantment.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_1_rule(q: &dyn Query, asker: &Asker, spell: ObjectId) -> bool { true }
#[allow(unused_variables)] fn ability_2_0_rule(q: &dyn Query, asker: &Asker, player: PlayerId, land: ObjectId, face: CardId) -> bool { !(player == asker.controller && (q.obj(land).zone.is(ZoneKind::Hand) && q.obj(land).owner == player) && true) }
#[allow(unused_variables)] fn ability_2_1_rule(q: &dyn Query, asker: &Asker, player: PlayerId, spell: ObjectId) -> bool { !(player == asker.controller && (q.obj(spell).zone.is(ZoneKind::Hand) && q.obj(spell).owner == player) && true) }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let destroyed_0 = g.destroy_all(&objects_0);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Experimental Frenzy", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_3, "{3}{R}: Destroy this enchantment.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, land_play_rules: &[ability_2_0_rule], play_rules: &[PlayRule::LookAtLibraryTop,PlayRule::LibraryTopLands,PlayRule::LibraryTopSpells(ability_1_1_rule, None)], spell_rules: &[ability_2_1_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
