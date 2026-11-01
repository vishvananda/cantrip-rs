// Jeweled Bird
// status: planned
// requires: card outside ordinary play
// requires: cost part ObjectAction
// requires: resolution cost ObjectAction
// requires: zone that belongs to no player
// Remove this card from your deck before playing if you're not playing for ante.
// {T}: Ante this artifact. If you do, put all other cards you own from the ante into your graveyard, then draw a card.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let paid_0 = planned_value!("resolution cost ObjectAction", false => cantrip_mechanics::effects::pay(g, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(planned_value!("cost part ObjectAction", Cost::NONE => Cost::object_action(Picks::Matching(|q: &dyn Query, asker: &Asker, id: ObjectId| id == asker.source), CostObjectAction::Ante))))?);
g.end_sentence();
if { let q: &dyn Query = g; ((paid_0 as i64) != 0) } {
planned!("zone-that-belongs-to-no-player", ChangeZone, "actor" = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, "cards" = "{\"args\":{\"filter\":{\"args\":{\"args\":{\"kind\":\"AbilityController\"},\"kind\":\"One\"},\"kind\":\"OwnedBy\"},\"kind\":\"Card\",\"scope\":{\"Shared\":\"Ante\"}},\"kind\":\"Zone\"}", "count" = "()", "destination" = "\"OwnersGraveyard\"", "entry" = "[]", "face_down" = "()", "group" = "()", "random" = "false");
g.end_sentence();
let amount_3 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_3 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_3 { g.draw(player, amount_3); }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
planned_item!("card-outside-ordinary-play" => deck_rules: DeckRule::OnlyWhenPlayingForAnte);

pub static CARD: CardDef = CardDef { name: "Jeweled Bird", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_1, "{T}: Ante this artifact. If you do, put all other cards you own from the ante into your graveyard, then draw a card.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
