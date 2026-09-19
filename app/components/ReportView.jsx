import { Fragment } from "react";

export default function ReportView({ model }) {
  const { accentColor, fixList, isReport, reportStats, reportSummary, sessionScore } = model;
  return (
    <>
      {" "}
      {isReport && (
        <>
          <div
            className="report-layout"
            style={{ display: "flex", flexDirection: "column", gap: "18px" }}
          >
            <div
              style={{
                display: "grid",
                gridTemplateColumns: "repeat(auto-fit,minmax(min(100%, 260px),1fr))",
                gap: "18px",
                alignItems: "stretch",
              }}
            >
              <div
                style={{
                  border: "1px solid #1c201c",
                  borderRadius: "14px",
                  padding: "22px",
                  display: "flex",
                  flexDirection: "column",
                  justifyContent: "space-between",
                  gap: "18px",
                }}
              >
                <div>
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
                    {"SESSION SCORE"}
                  </div>
                  <div
                    style={{
                      display: "flex",
                      alignItems: "baseline",
                      gap: "10px",
                      marginTop: "8px",
                    }}
                  >
                    <div
                      style={{
                        fontFamily: "'Barlow Condensed',sans-serif",
                        fontSize: "84px",
                        fontWeight: "700",
                        letterSpacing: "0",
                        lineHeight: ".85",
                        color: accentColor,
                      }}
                    >
                      {sessionScore}
                    </div>
                    <div
                      style={{
                        fontSize: "20px",
                        fontWeight: "600",
                        color: "#767d74",
                      }}
                    >
                      {"/ 100"}
                    </div>
                  </div>
                </div>
                <div
                  style={{
                    fontSize: "13px",
                    color: "#8d958b",
                    lineHeight: "1.55",
                    textWrap: "pretty",
                  }}
                >
                  {reportSummary}
                </div>
              </div>
              {reportStats.map((s, index) => (
                <Fragment key={index}>
                  <div
                    style={{
                      border: "1px solid #1c201c",
                      borderRadius: "14px",
                      padding: "18px",
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
                      {s.label}
                    </div>
                    <div
                      style={{
                        fontFamily: "'Barlow Condensed',sans-serif",
                        fontSize: "35px",
                        fontWeight: "700",
                        letterSpacing: "0",
                        marginTop: "9px",
                      }}
                    >
                      {s.value}
                    </div>
                    <div
                      style={{
                        fontSize: "11.5px",
                        color: "#767d74",
                        marginTop: "5px",
                        lineHeight: "1.45",
                      }}
                    >
                      {s.note}
                    </div>
                  </div>
                </Fragment>
              ))}
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
                {"FIX LIST \u00b7 NEXT SESSION"}
              </div>
              <div
                style={{
                  display: "flex",
                  flexDirection: "column",
                  gap: "13px",
                }}
              >
                {fixList.map((f, index) => (
                  <Fragment key={index}>
                    <div
                      style={{
                        display: "grid",
                        gridTemplateColumns: "26px minmax(0,1fr)",
                        gap: "12px",
                        alignItems: "start",
                        paddingBottom: "13px",
                        borderBottom: "1px solid #151815",
                      }}
                    >
                      <div
                        style={{
                          fontFamily: "'Barlow Condensed',sans-serif",
                          textTransform: "uppercase",
                          fontWeight: "600",
                          fontSize: "13.5px",
                          color: "#5e655d",
                          paddingTop: "2px",
                        }}
                      >
                        {f.n}
                      </div>
                      <div style={{ minWidth: "0" }}>
                        <div style={{ fontSize: "13.5px", fontWeight: "600" }}>{f.title}</div>
                        <div
                          style={{
                            fontSize: "12px",
                            color: "#8d958b",
                            lineHeight: "1.5",
                            marginTop: "3px",
                            textWrap: "pretty",
                          }}
                        >
                          {f.body}
                        </div>
                      </div>
                    </div>
                  </Fragment>
                ))}
              </div>
            </div>
          </div>
        </>
      )}{" "}
    </>
  );
}
