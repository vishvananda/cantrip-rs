// North Star
// status: planned
// requires: operation PlayWith
// requires: player rule for a while SpecialAction
// {4}, {T}: For one spell this turn, you may spend mana as though it were mana of any type to pay that spell's mana cost. (Additional costs are still paid normally.)
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("player-rule-for-a-while-special-action" => if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.add_player_effect(player, source, EffectKind::PlayerRule(PlayerRule::SpecialAction { body: |g: &mut dyn Game, asker: &Asker| -> Result<(), Suspend> { planned!("operation-play-with" => cantrip_mechanics::play::with_terms(g, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, asker.source, CardPlay::CastSpell, { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() }, { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }, None, "{\"limit\":null,\"during\":null,\"cost\":\"Normal\",\"spend_as\":\"AnyType\",\"into_exile\":false,\"condition\":null,\"effects\":[],\"flash\":false,\"may_look\":false,\"then\":null,\"face_down\":null}")?);
g.end_sentence();
 Ok(()) }, cost: None, limit: Some(PlayLimit::Total(1)), during: None, instant_speed: false }), Duration::EndOfTurn); } });
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "North Star", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_0, "{4}, {T}: For one spell this turn, you may spend mana as though it were mana of any type to pay that spell's mana cost.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
