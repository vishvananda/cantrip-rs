// Sphinx's Decree
// status: planned
// requires: duration DuringNextTurn
// requires: player rule for a while can't cast some spells
// Each opponent can't cast instant or sorcery spells during that player's next turn.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
{ let order: Vec<PlayerId> = g.state().apnap().collect(); players_0.sort_by_key(|p| order.iter().position(|o| o == p)); }
g.one_instruction(&mut |g| { for (index_0, bound_0) in players_0.iter().copied().enumerate() { planned!("player-rule-for-a-while-can-t-cast-some-spells" => let players_1_0 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() }; if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { for &player in &players_1_0 { g.add_player_effect(asker.controller, source, EffectKind::CantCast { player, spells: SpellSet::matching(move |q, spell| (q.chars(spell).is(CardType::Instant) || q.chars(spell).is(CardType::Sorcery))) }, planned_value!("duration DuringNextTurn", Duration::Indefinite => Duration::DuringNextTurn(Bound(Binding(0))))); } });
    } Ok(()) })?;
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Sphinx's Decree", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
