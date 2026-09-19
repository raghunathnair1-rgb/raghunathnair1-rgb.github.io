import { Fragment } from "react";

export default function LibraryView({ model }) {
  const { isLib, library } = model;
  return (
    <>
      {" "}
      {isLib && (
        <>
          <div
            className="library-layout"
            style={{
              display: "grid",
              gridTemplateColumns: "repeat(auto-fill,minmax(min(100%, 236px),1fr))",
              gap: "16px",
            }}
          >
            {library.map((m, index) => (
              <Fragment key={index}>
                <button
                  onClick={m.go}
                  style={{
                    border: "1px solid " + m.border,
                    borderRadius: "14px",
                    padding: "16px 17px",
                    cursor: "pointer",
                    display: "flex",
                    flexDirection: "column",
                    gap: "13px",
                    minHeight: "196px",
                  }}
                  type="button"
                  className="hover-2"
                >
                  <div
                    style={{
                      height: "74px",
                      borderRadius: "9px",
                      background:
                        "repeating-linear-gradient(135deg,#111311 0 8px,#0d0f0d 8px 16px)",
                      display: "flex",
                      alignItems: "flex-end",
                      padding: "8px",
                    }}
                  >
                    <span
                      style={{
                        fontFamily: "'Barlow Condensed',sans-serif",
                        textTransform: "uppercase",
                        fontWeight: "600",
                        fontSize: "10.5px",
                        color: "#5e655d",
                        letterSpacing: "0.12em",
                      }}
                    >
                      {m.slot}
                    </span>
                  </div>
                  <div
                    style={{
                      display: "flex",
                      alignItems: "flex-start",
                      justifyContent: "space-between",
                      gap: "10px",
                    }}
                  >
                    <div style={{ minWidth: "0" }}>
                      <div
                        style={{
                          fontSize: "15px",
                          fontWeight: "600",
                          letterSpacing: "-0.02em",
                        }}
                      >
                        {m.name}
                      </div>
                      <div
                        style={{
                          fontFamily: "'Barlow Condensed',sans-serif",
                          textTransform: "uppercase",
                          fontWeight: "600",
                          fontSize: "12px",
                          color: "#767d74",
                          letterSpacing: "0.1em",
                          marginTop: "4px",
                        }}
                      >
                        {m.tier}
                      </div>
                    </div>
                    <div
                      style={{
                        fontSize: "18px",
                        fontWeight: "700",
                        letterSpacing: "-0.03em",
                        color: m.gradeColor,
                      }}
                    >
                      {m.grade}
                    </div>
                  </div>
                  <div style={{ marginTop: "auto" }}>
                    <div
                      style={{
                        display: "flex",
                        justifyContent: "space-between",
                        fontFamily: "'Barlow Condensed',sans-serif",
                        textTransform: "uppercase",
                        fontWeight: "600",
                        fontSize: "11.5px",
                        color: "#767d74",
                        marginBottom: "6px",
                      }}
                    >
                      <span>{"MASTERY"}</span>
                      <span>{m.mastery}</span>
                    </div>
                    <div
                      style={{
                        height: "4px",
                        background: "#161a16",
                        borderRadius: "2px",
                        overflow: "hidden",
                      }}
                    >
                      <div
                        style={{
                          width: m.mastery,
                          height: "100%",
                          background: m.gradeColor,
                        }}
                      ></div>
                    </div>
                  </div>
                </button>
              </Fragment>
            ))}
          </div>
        </>
      )}{" "}
    </>
  );
}
