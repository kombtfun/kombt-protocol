import { KombtError } from "../src/errors";

describe("kombt errors", () => {
  it("invalidStake returns code InvalidStake", () => {
    const e = KombtError.invalidStake();
    expect(e).toBeInstanceOf(KombtError);
    expect(e.code).toBe("InvalidStake");
    expect(e.message).toContain("stake");
  });

  it("expired returns code Expired", () => {
    const e = KombtError.expired();
    expect(e.code).toBe("Expired");
    expect(e.message).toContain("expired");
  });

  it("unauthorized returns code Unauthorized", () => {
    const e = KombtError.unauthorized();
    expect(e.code).toBe("Unauthorized");
    expect(e.message).toContain("authorized");
  });

  it("is a real Error subclass", () => {
    const e = KombtError.invalidStake();
    expect(e instanceof Error).toBe(true);
    expect(e.name).toBe("KombtError");
  });
});

// rev18

// rev25

// rev37

// rev59

// rev114

// rev130

// rev144

// rev148
