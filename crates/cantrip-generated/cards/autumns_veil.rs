// Autumn's Veil
// status: planned
// requires: continuous rule Prohibit BeTargeted
// requires: engine: no resolving rule on an open set: Prohibit BeTargeted
// requires: player rule for a while SpellsHaveAbilities
// Spells you control can't be countered by blue or black spells this turn, and creatures you control can't be the targets of blue or black spells this turn.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("player-rule-for-a-while-spells-have-abilities" => if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.add_player_effect(player, source, EffectKind::PlayerRule(PlayerRule::SpellsHave(SpellGrant { spells: |q: &dyn Query, asker: &Asker, player: PlayerId, spell: ObjectId| true, from: None, nth_each_turn: None, grants: &[SpellEffect::CantBeCounteredBySpells(|q: &dyn Query, asker: &Asker, id: ObjectId| (q.chars(id).colors.has(q.words(asker.source).color(Color::Blue)) || q.chars(id).colors.has(q.words(asker.source).color(Color::Black))))] })), Duration::EndOfTurn); } });
g.end_sentence();
planned!("engine-no-resolving-rule-on-an-open-set-prohibit-be-targeted" => if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { g.add_open_effect(source, move |q: &dyn Query, asker: &Asker, id: ObjectId| (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))), planned_value!("continuous rule Prohibit BeTargeted", EffectKind::CantBlock => EffectKind::Rule(Rule::Cant(Action::BeTargeted(|q: &dyn Query, asker: &Asker, by: &Asker, by_source: &Chars| (by.spell && ((*by_source).colors.has(q.words(asker.source).color(Color::Blue)) || (*by_source).colors.has(q.words(asker.source).color(Color::Black)))))))), Duration::EndOfTurn); });
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Autumn's Veil", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
