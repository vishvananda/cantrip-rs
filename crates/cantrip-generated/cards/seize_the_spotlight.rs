// Seize the Spotlight
// status: planned
// requires: condition VotedFor
// requires: object reference Bound
// requires: operation EachPlayer
// requires: per-player EachPlayer
// Each opponent chooses fame or fortune. For each player who chose fame, gain control of a creature that player controls until end of turn. Untap those creatures and they gain haste until end of turn. For each player who chose fortune, you draw a card and create a Treasure token.
#[allow(unused_imports)] use cantrip_api::*;
pub mod token_10 {
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_0 {
let color = chosen.expect("mana activation selected a color");
g.add_mana(player, color.mana(), mana_0);}
 Ok(()) }

pub static CARD: CardDef = CardDef { name: token!(Treasure).name, token_words: token!(Treasure).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[Subtype::Treasure]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(SACRIFICE_THIS).and(Cost { tap: true, ..Cost::NONE }), add_by(ability_0_mana).choose_color().colors(ColorSet(31)).yields_any(1), "Add mana.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
let mut each_chosen_1 = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let player_1 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let card_1 = g.card_id(&CARD); let chosen_1: u8 = cantrip_mechanics::choose::option(g, player_1, asker.source, card_1, 0..2)?;
 each_chosen_1.push(chosen_1);
   } 
g.end_sentence();
let mut players_2: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| planned_value!("condition VotedFor", false => g.votes().voted(option))).collect::<Vec<PlayerId>>() };
planned!("per-player-each-player" => g.begin_batch(););
 for (index_2, bound_9000) in players_2.iter().copied().enumerate() { planned!("operation-each-player", EachPlayer, "action" = "{\"args\":{\"objects\":{\"args\":{\"chooser\":{\"args\":{\"kind\":\"AbilityController\"},\"kind\":\"One\"},\"count\":{\"Exactly\":{\"args\":1,\"kind\":\"Constant\"}},\"objects\":{\"args\":{\"filter\":{\"args\":{\"args\":{\"kind\":\"Acting\"},\"kind\":\"One\"},\"kind\":\"ControlledBy\"},\"objects\":{\"args\":{\"args\":\"Creature\",\"kind\":\"CardType\"},\"kind\":\"Battlefield\"}},\"kind\":\"Matching\"}},\"kind\":\"Chosen\"},\"players\":{\"args\":{\"kind\":\"AbilityController\"},\"kind\":\"One\"},\"until\":\"EndOfTurn\"},\"kind\":\"GiveControl\"}", "players" = { let q: &dyn Query = g; planned_value!("object reference Bound", None::<ObjectId> => bound_9000).into_iter().collect::<Vec<ObjectId>>() });
    } 
planned!("per-player-each-player" => g.end_batch(););
g.end_sentence();
let objects_6 = { let q: &dyn Query = g; (planned_value!("depends on an instruction with a gap", Vec::<ObjectId>::new() => result_1)).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let turned_6: Vec<ObjectId> = objects_6.iter().copied().filter(|&o| g.state().obj(o).tapped != false).collect(); cantrip_api::instruction(g, |g| { for &object in &turned_6 { g.untap(object); } });
g.end_sentence();
let objects_7 = { let q: &dyn Query = g; (planned_value!("depends on an instruction with a gap", Vec::<ObjectId>::new() => result_1)).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_7 = { let q: &dyn Query = g; Some(asker.source) }; let duration_7 = Duration::EndOfTurn;
let kind_7_0_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Haste])) };
if let Some(source) = source_7 { cantrip_api::instruction(g, |g| { for &object in &objects_7 { g.add_effect(source, object, kind_7_0_0, duration_7);  } }); }
g.end_sentence();
let amount_8 = { let q: &dyn Query = g; ((q.state().apnap().filter(|&bound_0: &PlayerId| planned_value!("condition VotedFor", false => g.votes().voted(option))).collect::<Vec<PlayerId>>()).len() as i64).max(0) as u32 }; let players_8 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_8 { g.draw(player, amount_8); }
g.end_sentence();
let players_9 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_9 = Vec::new();
let count_9_0 = { let q: &dyn Query = g; ((q.state().apnap().filter(|&bound_0: &PlayerId| planned_value!("condition VotedFor", false => g.votes().voted(option))).collect::<Vec<PlayerId>>()).len() as i64).max(0) as u32 }; let def_9_0 = g.card_id(&token_10::CARD);
for player in players_9 {
tokens_9.extend(std::iter::repeat((def_9_0,player)).take(count_9_0 as usize));
} let words_9 = Words::NONE;
let created_9 = g.create_tokens_worded(&tokens_9, false, None, words_9)?;
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Seize the Spotlight", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, labels: &["Fame", "Fortune"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_10::CARD];
