// Leonin Surveyor
// status: ready
// Start your engines! (If you have no speed, it starts at 1. It increases once on each of your turns when an opponent loses life. Max speed is 4.)
// During your turn, this creature has first strike.
// Max speed — {3}, Exile this card from your graveyard: Draw a card.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (Some(asker.controller) == Some(q.state().turn.active)) && (source == affected) }
#[allow(unused_variables)] fn ability_1_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::FirstStrike); }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.draw(player, amount_0); }
g.end_sentence();
 Ok(()) }
fn ability_2_if(q: &dyn Query, asker: &Asker) -> bool { Some(asker.controller).is_some_and(|player: PlayerId| q.state().player(player).speed == Some(4)) }

pub static CARD: CardDef = CardDef { name: "Leonin Surveyor", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Cat,Subtype::Scout]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(EXILE_THIS), ability_2, "Max speed — {3}, Exile this card from your graveyard: Draw a card.").targets(&[]).condition(ability_2_if).from(ZoneKind::Graveyard)], statics: &[static_ability(Layer::Ability, ability_1_0_affects, ability_1_0_apply_0, "During your turn, this creature has first strike.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, start_your_engines: true, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
