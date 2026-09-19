import { Fragment } from "react";

export default function DashboardView({ model }) {
  const { isDash, moveRows, tiles, trend, weakPoints } = model;
  return (
    <>
      {" "}
      {isDash && (
        <>
          <div
            className="dashboard-layout"
            style={{ display: "flex", flexDirection: "column", gap: "18px" }}
          >
            <div
              style={{
                display: "grid",
                gridTemplateColumns: "repeat(auto-fit,minmax(min(100%, 180px),1fr))",
                gap: "14px",
              }}
            >
              {tiles.map((t, index) => (
                <Fragment key={index}>
                  <div
                    style={{
                      border: "1px solid #1c201c",
                      borderRadius: "14px",
                      padding: "17px 18px",
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
                      {t.label}
                    </div>
                    <div
                      style={{
                        display: "flex",
                        alignItems: "baseline",
                        gap: "7px",
                        marginTop: "9px",
                      }}
                    >
                      <div
                        style={{
                          fontFamily: "'Barlow Condensed',sans-serif",
                          fontSize: "37px",
                          fontWeight: "700",
                          letterSpacing: "0",
                        }}
                      >
                        {t.value}
                      </div>
                      <div
                        style={{
                          fontSize: "11.5px",
                          fontWeight: "600",
                          color: t.deltaColor,
                        }}
                      >
                        {t.delta}
                      </div>
                    </div>
                    <div
                      style={{
                        fontSize: "11.5px",
                        color: "#767d74",
                        marginTop: "5px",
                      }}
                    >
                      {t.note}
                    </div>
                  </div>
                </Fragment>
              ))}
            </div>
            <div
              style={{
                display: "grid",
                gridTemplateColumns: "repeat(auto-fit,minmax(min(100%, 320px),1fr))",
                gap: "18px",
                alignItems: "start",
              }}
            >
              <div
                style={{
                  border: "1px solid #1c201c",
                  borderRadius: "14px",
                  padding: "18px 20px",
                }}
              >
                <div
                  style={{
                    display: "flex",
                    alignItems: "baseline",
                    justifyContent: "space-between",
                    gap: "12px",
                    flexWrap: "wrap",
                    marginBottom: "18px",
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
                    {"ACCURACY TREND \u00b7 12 WEEKS"}
                  </div>
                  <div
                    style={{
                      fontFamily: "'Barlow Condensed',sans-serif",
                      textTransform: "uppercase",
                      fontWeight: "600",
                      fontSize: "13px",
                      color: "#5fe08f",
                    }}
                  >
                    {"+6.4 PTS"}
                  </div>
                </div>
                <div
                  style={{
                    display: "flex",
                    alignItems: "flex-end",
                    gap: "6px",
                    height: "150px",
                  }}
                >
                  {trend.map((b, index) => (
                    <Fragment key={index}>
                      <div
                        style={{
                          flex: "1",
                          minWidth: "0",
                          display: "flex",
                          flexDirection: "column",
                          justifyContent: "flex-end",
                          gap: "6px",
                          height: "100%",
                        }}
                      >
                        <div
                          style={{
                            height: b.h,
                            background: b.color,
                            borderRadius: "3px",
                          }}
                          className="hover-0"
                        ></div>
                        <div
                          style={{
                            fontFamily: "'Barlow Condensed',sans-serif",
                            textTransform: "uppercase",
                            fontWeight: "600",
                            fontSize: "10.5px",
                            color: "#5e655d",
                            textAlign: "center",
                          }}
                        >
                          {b.label}
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
                  padding: "18px 20px",
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
                    marginBottom: "16px",
                  }}
                >
                  {"WEAK POINTS DETECTED"}
                </div>
                <div
                  style={{
                    display: "flex",
                    flexDirection: "column",
                    gap: "14px",
                  }}
                >
                  {weakPoints.map((w, index) => (
                    <Fragment key={index}>
                      <div
                        style={{
                          display: "flex",
                          gap: "13px",
                          alignItems: "flex-start",
                          paddingBottom: "14px",
                          borderBottom: "1px solid #151815",
                        }}
                      >
                        <div
                          style={{
                            fontFamily: "'Barlow Condensed',sans-serif",
                            textTransform: "uppercase",
                            fontWeight: "600",
                            fontSize: "14.5px",
                            fontWeight: "700",
                            color: w.color,
                            flex: "0 0 auto",
                            paddingTop: "1px",
                          }}
                        >
                          {w.score}
                        </div>
                        <div style={{ minWidth: "0" }}>
                          <div style={{ fontSize: "13.5px", fontWeight: "600" }}>{w.title}</div>
                          <div
                            style={{
                              fontSize: "12px",
                              color: "#8d958b",
                              lineHeight: "1.5",
                              marginTop: "3px",
                              textWrap: "pretty",
                            }}
                          >
                            {w.body}
                          </div>
                        </div>
                      </div>
                    </Fragment>
                  ))}
                </div>
              </div>
            </div>
            <div
              className="grade-table"
              style={{
                border: "1px solid #1c201c",
                borderRadius: "14px",
                padding: "18px 20px",
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
                  marginBottom: "16px",
                }}
              >
                {"PER-MOVE GRADES"}
              </div>
              <div
                style={{
                  display: "grid",
                  gridTemplateColumns: "minmax(110px,1.4fr) repeat(4,minmax(56px,.7fr)) 52px",
                  minWidth: "480px",
                  width: "100%",
                  gap: "10px",
                  paddingBottom: "10px",
                  borderBottom: "1px solid #1c201c",
                  fontFamily: "'Barlow Condensed',sans-serif",
                  textTransform: "uppercase",
                  fontWeight: "600",
                  fontSize: "11.5px",
                  color: "#767d74",
                  letterSpacing: "0.12em",
                }}
              >
                <div>{"MOVE"}</div>
                <div>{"FORM"}</div>
                <div>{"ROM"}</div>
                <div>{"TEMPO"}</div>
                <div>{"SYMM"}</div>
                <div style={{ textAlign: "right" }}>{"GRADE"}</div>
              </div>
              {moveRows.map((m, index) => (
                <Fragment key={index}>
                  <button
                    onClick={m.go}
                    style={{
                      display: "grid",
                      gridTemplateColumns: "minmax(110px,1.4fr) repeat(4,minmax(56px,.7fr)) 52px",
                      minWidth: "480px",
                      width: "100%",
                      gap: "10px",
                      padding: "12px 0",
                      borderBottom: "1px solid #151815",
                      cursor: "pointer",
                      alignItems: "center",
                    }}
                    type="button"
                    className="hover-1"
                  >
                    <div
                      style={{
                        fontSize: "13.5px",
                        fontWeight: "500",
                        whiteSpace: "nowrap",
                        overflow: "hidden",
                        textOverflow: "ellipsis",
                      }}
                    >
                      {m.name}
                    </div>
                    <div
                      style={{
                        fontFamily: "'Barlow Condensed',sans-serif",
                        textTransform: "uppercase",
                        fontWeight: "600",
                        fontSize: "14.5px",
                        color: m.formColor,
                      }}
                    >
                      {m.form}
                    </div>
                    <div
                      style={{
                        fontFamily: "'Barlow Condensed',sans-serif",
                        textTransform: "uppercase",
                        fontWeight: "600",
                        fontSize: "14.5px",
                        color: "#a8b0a6",
                      }}
                    >
                      {m.rom}
                    </div>
                    <div
                      style={{
                        fontFamily: "'Barlow Condensed',sans-serif",
                        textTransform: "uppercase",
                        fontWeight: "600",
                        fontSize: "14.5px",
                        color: "#a8b0a6",
                      }}
                    >
                      {m.tempo}
                    </div>
                    <div
                      style={{
                        fontFamily: "'Barlow Condensed',sans-serif",
                        textTransform: "uppercase",
                        fontWeight: "600",
                        fontSize: "14.5px",
                        color: "#a8b0a6",
                      }}
                    >
                      {m.symm}
                    </div>
                    <div
                      style={{
                        textAlign: "right",
                        fontSize: "14px",
                        fontWeight: "700",
                        color: m.gradeColor,
                      }}
                    >
                      {m.grade}
                    </div>
                  </button>
                </Fragment>
              ))}
            </div>
          </div>
        </>
      )}{" "}
    </>
  );
}
