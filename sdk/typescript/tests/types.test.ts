import { MatchType, MatchState, StakeToken } from "../src/types";

describe("kombt sdk type tags", () => {
  it("MatchType enum has Price=0, Time=1, Vrf=2", () => {
    expect(MatchType.Price).toBe(0);
    expect(MatchType.Time).toBe(1);
    expect(MatchType.Vrf).toBe(2);
  });

  it("StakeToken enum has Kombt=0, Sol=1, Usdc=2", () => {
    expect(StakeToken.Kombt).toBe(0);
    expect(StakeToken.Sol).toBe(1);
    expect(StakeToken.Usdc).toBe(2);
  });

  it("MatchState enum has Pending=0, Active=1, Settled=2, Cancelled=3", () => {
    expect(MatchState.Pending).toBe(0);
    expect(MatchState.Active).toBe(1);
    expect(MatchState.Settled).toBe(2);
    expect(MatchState.Cancelled).toBe(3);
  });
});

// rev5

// rev8

// rev27

// rev37

// rev38

// rev58

// rev59

// rev62

// rev68

// rev75

// rev85
