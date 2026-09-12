import { Fragment } from "react";

export default function SessionView({ model }) {
  const {
    accDeg,
    accentColor,
    accuracy,
    cues,
    focusColor,
    focusX,
    focusY,
    isSession,
    jointRows,
    joints,
    liveChips,
    miniStats,
    moveName,
    phaseColor,
    phaseLabel,
    repBars,
    repProgress,
    verdict,
    verdictColor,
    verdictNote,
  } = model;
  return (
    <>
      {" "}
      {isSession && (
        <>
          <div
            style={{
              display: "grid",
              gridTemplateColumns:
                "repeat(auto-fit,minmax(min(100%, 360px),1fr))",
              gap: "22px",
              alignItems: "start",
            }}
          >
            <div
              style={{
                display: "flex",
                flexDirection: "column",
                gap: "18px",
                minWidth: "0",
              }}
            >
              <div
                style={{
                  position: "relative",
                  border: "1px solid #1c201c",
                  borderRadius: "14px",
                  overflow: "hidden",
                  background:
                    "repeating-linear-gradient(135deg,#101210 0 9px,#0c0e0c 9px 18px)",
                  aspectRatio: "16/10",
                  minHeight: "320px",
                  width: "100%",
                  maxWidth: "100%",
                  minWidth: "0",
                }}
              >
                <div
                  style={{
                    position: "absolute",
                    left: "0",
                    right: "0",
                    height: "1px",
                    background:
                      "linear-gradient(90deg,transparent,#5fe08f,transparent)",
                    animation: "scan 3.4s linear infinite",
                  }}
                ></div>
                {joints.map((j, index) => (
                  <Fragment key={index}>
                    <div
                      style={{
                        position: "absolute",
                        left: j.x,
                        top: j.y,
                        width: j.size,
                        height: j.size,
                        marginLeft: "-5px",
                        marginTop: "-5px",
                        borderRadius: "50%",
                        background: j.color,
                        boxShadow: "0 0 0 3px " + j.halo,
                      }}
                    ></div>
                  </Fragment>
                ))}
                <div
                  style={{
                    position: "absolute",
                    left: focusX,
                    top: focusY,
                    width: "52px",
                    height: "52px",
                    marginLeft: "-26px",
                    marginTop: "-26px",
                    border: "1px solid " + focusColor,
                    borderRadius: "50%",
                    animation: "pulseDot 1.4s ease-in-out infinite",
                  }}
                ></div>
                <div
                  style={{
                    position: "absolute",
                    top: "14px",
                    left: "14px",
                    display: "flex",
                    gap: "7px",
                    alignItems: "center",
                  }}
                >
                  <div
                    style={{
                      fontFamily: "'Barlow Condensed',sans-serif",
                      textTransform: "uppercase",
                      fontWeight: "600",
                      fontSize: "12px",
                      letterSpacing: "0.14em",
                      color: "#767d74",
                      background: "#0a0b0acc",
                      border: "1px solid #1c201c",
                      borderRadius: "6px",
                      padding: "5px 8px",
                    }}
                  >
                    {"SIMULATED FEED · DEMO"}
                  </div>
                </div>
                <div
                  style={{
                    position: "absolute",
                    bottom: "14px",
                    left: "14px",
                    right: "14px",
                    display: "flex",
                    alignItems: "flex-end",
                    justifyContent: "space-between",
                    gap: "14px",
                    flexWrap: "wrap",
                  }}
                >
                  <div
                    style={{
                      background: "#0a0b0ae6",
                      border: "1px solid #1c201c",
                      borderRadius: "11px",
                      padding: "12px 15px",
                      backdropFilter: "blur(4px)",
                    }}
                  >
                    <div
                      style={{
                        fontFamily: "'Barlow Condensed',sans-serif",
                        textTransform: "uppercase",
                        fontWeight: "600",
                        fontSize: "11.5px",
                        color: "#767d74",
                        letterSpacing: "0.14em",
                      }}
                    >
                      {"CURRENT MOVE"}
                    </div>
                    <div
                      style={{
                        fontSize: "19px",
                        fontWeight: "700",
                        letterSpacing: "-0.03em",
                        marginTop: "3px",
                      }}
                    >
                      {moveName}
                    </div>
                    <div
                      style={{
                        fontFamily: "'Barlow Condensed',sans-serif",
                        textTransform: "uppercase",
                        fontWeight: "600",
                        fontSize: "12.5px",
                        color: phaseColor,
                        marginTop: "4px",
                      }}
                    >
                      {phaseLabel}
                    </div>
                  </div>
                  <div
                    className="live-chips"
                    style={{ display: "flex", gap: "9px" }}
                  >
                    {liveChips.map((c, index) => (
                      <Fragment key={index}>
                        <div
                          style={{
                            background: "#0a0b0ae6",
                            border: "1px solid #1c201c",
                            borderRadius: "11px",
                            padding: "10px 13px",
                            minWidth: "82px",
                          }}
                        >
                          <div
                            style={{
                              fontFamily: "'Barlow Condensed',sans-serif",
                              textTransform: "uppercase",
                              fontWeight: "600",
                              fontSize: "11px",
                              color: "#767d74",
                              letterSpacing: "0.14em",
                            }}
                          >
                            {c.label}
                          </div>
                          <div
                            style={{
                              fontSize: "17px",
                              fontWeight: "700",
                              letterSpacing: "-0.03em",
                              marginTop: "2px",
                              color: c.color,
                            }}
                          >
                            {c.value}
                          </div>
                        </div>
                      </Fragment>
                    ))}
                  </div>
                </div>
              </div>
              <div
                style={{
                  border: "1px solid #1c201c",
                  borderRadius: "14px",
                  padding: "16px 18px",
                }}
              >
                <div
                  style={{
                    display: "flex",
                    alignItems: "baseline",
                    justifyContent: "space-between",
                    gap: "14px",
                    marginBottom: "14px",
                    flexWrap: "wrap",
                  }}
                >
                  <div
                    style={{
                      fontFamily: "'Barlow Condensed',sans-serif",
                      textTransform: "uppercase",
                      fontWeight: "600",
                      fontSize: "12px",
                      color: "#767d74",
                      letterSpacing: "0.16em",
                    }}
                  >
                    {"REP-BY-REP ACCURACY"}
                  </div>
                  <div
                    style={{
                      fontFamily: "'Barlow Condensed',sans-serif",
                      textTransform: "uppercase",
                      fontWeight: "600",
                      fontSize: "13px",
                      color: "#a8b0a6",
                    }}
                  >
                    {repProgress}
                  </div>
                </div>
                <div
                  style={{
                    display: "flex",
                    alignItems: "flex-end",
                    gap: "5px",
                    height: "104px",
                  }}
                >
                  {repBars.map((r, index) => (
                    <Fragment key={index}>
                      <div
                        style={{
                          flex: "1",
                          minWidth: "0",
                          display: "flex",
                          flexDirection: "column",
                          justifyContent: "flex-end",
                          gap: "5px",
                          height: "100%",
                        }}
                      >
                        <div
                          style={{
                            height: r.h,
                            background: r.color,
                            borderRadius: "3px",
                            minHeight: "3px",
                            animation: "riseIn .35s ease",
                          }}
                        ></div>
                        <div
                          style={{
                            fontFamily: "'Barlow Condensed',sans-serif",
                            textTransform: "uppercase",
                            fontWeight: "600",
                            fontSize: "11px",
                            color: "#5e655d",
                            textAlign: "center",
                          }}
                        >
                          {r.n}
                        </div>
                      </div>
                    </Fragment>
                  ))}
                </div>
              </div>
            </div>
            <div
              style={{
                display: "flex",
                flexDirection: "column",
                gap: "18px",
                minWidth: "0",
              }}
            >
              <div
                style={{
                  border: "1px solid #1c201c",
                  borderRadius: "14px",
                  padding: "20px",
                  display: "flex",
                  alignItems: "center",
                  gap: "20px",
                  flexWrap: "wrap",
                }}
              >
                <div
                  style={{
                    position: "relative",
                    width: "124px",
                    height: "124px",
                    borderRadius: "50%",
                    background:
                      "conic-gradient(" +
                      accentColor +
                      " " +
                      accDeg +
                      ", #161a16 0)",
                    display: "flex",
                    alignItems: "center",
                    justifyContent: "center",
                    flex: "0 0 auto",
                  }}
                >
                  <div
                    style={{
                      width: "100px",
                      height: "100px",
                      borderRadius: "50%",
                      background: "#0a0b0a",
                      display: "flex",
                      flexDirection: "column",
                      alignItems: "center",
                      justifyContent: "center",
                    }}
                  >
                    <div
                      style={{
                        fontFamily: "'Barlow Condensed',sans-serif",
                        fontSize: "38px",
                        fontWeight: "700",
                        letterSpacing: "0",
                        lineHeight: "1",
                      }}
                    >
                      {accuracy}
                    </div>
                    <div
                      style={{
                        fontFamily: "'Barlow Condensed',sans-serif",
                        textTransform: "uppercase",
                        fontWeight: "600",
                        fontSize: "11px",
                        color: "#767d74",
                        letterSpacing: "0.14em",
                        marginTop: "3px",
                      }}
                    >
                      {"FORM SCORE"}
                    </div>
                  </div>
                </div>
                <div
                  style={{
                    flex: "1",
                    minWidth: "150px",
                    display: "flex",
                    flexDirection: "column",
                    gap: "9px",
                  }}
                >
                  <div
                    style={{
                      fontSize: "13.5px",
                      fontWeight: "600",
                      color: verdictColor,
                    }}
                  >
                    {verdict}
                  </div>
                  <div
                    style={{
                      fontSize: "12.5px",
                      lineHeight: "1.5",
                      color: "#8d958b",
                      textWrap: "pretty",
                    }}
                  >
                    {verdictNote}
                  </div>
                  <div
                    style={{ display: "flex", gap: "14px", marginTop: "2px" }}
                  >
                    {miniStats.map((m, index) => (
                      <Fragment key={index}>
                        <div>
                          <div
                            style={{
                              fontFamily: "'Barlow Condensed',sans-serif",
                              textTransform: "uppercase",
                              fontWeight: "600",
                              fontSize: "11px",
                              color: "#767d74",
                              letterSpacing: "0.12em",
                            }}
                          >
                            {m.label}
                          </div>
                          <div
                            style={{
                              fontSize: "14px",
                              fontWeight: "600",
                              marginTop: "2px",
                            }}
                          >
                            {m.value}
                          </div>
                        </div>
                      </Fragment>
                    ))}
                  </div>
                </div>
              </div>
              <div
                style={{
                  border: "1px solid #1c201c",
                  borderRadius: "14px",
                  padding: "16px 18px",
                }}
              >
                <div
                  style={{
                    fontFamily: "'Barlow Condensed',sans-serif",
                    textTransform: "uppercase",
                    fontWeight: "600",
                    fontSize: "12px",
                    color: "#767d74",
                    letterSpacing: "0.16em",
                    marginBottom: "14px",
                  }}
                >
                  {"JOINT ANGLE DEVIATION"}
                </div>
                <div
                  style={{
                    display: "flex",
                    flexDirection: "column",
                    gap: "11px",
                  }}
                >
                  {jointRows.map((row, index) => (
                    <Fragment key={index}>
                      <div
                        style={{
                          display: "grid",
                          gridTemplateColumns: "92px minmax(0,1fr) 44px",
                          alignItems: "center",
                          gap: "10px",
                        }}
                      >
                        <div
                          style={{
                            fontSize: "12px",
                            color: "#a8b0a6",
                            whiteSpace: "nowrap",
                            overflow: "hidden",
                            textOverflow: "ellipsis",
                          }}
                        >
                          {row.name}
                        </div>
                        <div
                          style={{
                            height: "6px",
                            background: "#161a16",
                            borderRadius: "3px",
                            overflow: "hidden",
                          }}
                        >
                          <div
                            style={{
                              width: row.pct,
                              height: "100%",
                              background: row.color,
                              borderRadius: "3px",
                            }}
                          ></div>
                        </div>
                        <div
                          style={{
                            fontFamily: "'Barlow Condensed',sans-serif",
                            textTransform: "uppercase",
                            fontWeight: "600",
                            fontSize: "13.5px",
                            textAlign: "right",
                            color: row.color,
                          }}
                        >
                          {row.dev}
                        </div>
                      </div>
                    </Fragment>
                  ))}
                </div>
              </div>
              <div
                style={{
                  border: "1px solid #1c201c",
                  borderRadius: "14px",
                  padding: "16px 18px",
                }}
              >
                <div
                  style={{
                    fontFamily: "'Barlow Condensed',sans-serif",
                    textTransform: "uppercase",
                    fontWeight: "600",
                    fontSize: "12px",
                    color: "#767d74",
                    letterSpacing: "0.16em",
                    marginBottom: "14px",
                  }}
                >
                  {"LIVE COACHING CUES"}
                </div>
                <div
                  style={{
                    display: "flex",
                    flexDirection: "column",
                    gap: "10px",
                  }}
                >
                  {cues.map((c, index) => (
                    <Fragment key={index}>
                      <div
                        style={{
                          display: "flex",
                          gap: "10px",
                          alignItems: "flex-start",
                          animation: "riseIn .3s ease",
                        }}
                      >
                        <div
                          style={{
                            width: "5px",
                            height: "5px",
                            borderRadius: "50%",
                            background: c.color,
                            marginTop: "6px",
                            flex: "0 0 auto",
                          }}
                        ></div>
                        <div style={{ minWidth: "0" }}>
                          <div
                            style={{
                              fontSize: "12.5px",
                              fontWeight: "600",
                              color: c.color,
                            }}
                          >
                            {c.head}
                          </div>
                          <div
                            style={{
                              fontSize: "12px",
                              color: "#8d958b",
                              lineHeight: "1.45",
                              textWrap: "pretty",
                            }}
                          >
                            {c.body}
                          </div>
                        </div>
                      </div>
                    </Fragment>
                  ))}
                </div>
              </div>
            </div>
          </div>
        </>
      )}{" "}
    </>
  );
}
