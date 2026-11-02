// Orb of Dragonkind
// status: ready
// {1}, {T}: Add two mana in any combination of colors. Spend this mana only to cast Dragon spells or activate abilities of Dragons.
// {R}, {T}, Sacrifice this artifact: Look at the top seven cards of your library. You may reveal a Dragon card from among them and put it into your hand. Put the rest on the bottom of your library in a random order.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_0 = { let q: &dyn Query = g; (2i64).max(0) as u8 };
for player in mana_players_0 {
if mana_0 > 0 { let colors = cantrip_mechanics::choose_colors(g, player, asker.source, ColorSet(31), mana_0)?;
for color in colors { g.add_restricted_mana(player, color.mana(), 1, ManaRestriction::Subtypes(SubtypeSpells::of(g.words(asker.source).subtype(Subtype::Dragon)).or_abilities_of_permanents())); }
}
}
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let depth_1 = { let q: &dyn Query = g; (7i64).max(0) as usize }; let word_1_0 = { let q: &dyn Query = g; q.words(asker.source).subtype(Subtype::Dragon) }; let taken_1 = cantrip_mechanics::library::look_at_top(g, player_1, asker.source, depth_1, (0, 1), true, |c| c.has_subtype(word_1_0), cantrip_mechanics::library::Rest::BottomRandom)?;
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Orb of Dragonkind", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), add_by(ability_0_mana).colors(ColorSet(31)).yields_any(2).only_for(ManaRestriction::Subtypes(SubtypeSpells::of(Subtype::Dragon).or_abilities_of_permanents())), "{1}, {T}: Add two mana in any combination of colors. Spend this mana only to cast Dragon spells or activate abilities of Dragons."),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(SACRIFICE_THIS).and(Cost { tap: true, ..Cost::NONE }), ability_1, "{R}, {T}, Sacrifice this artifact: Look at the top seven cards of your library. You may reveal a Dragon card from among them and put it into your hand. Put the rest on the bottom of your library in a random order.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
