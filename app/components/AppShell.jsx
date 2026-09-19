import { Fragment } from "react";
import SessionView from "./SessionView";
import DashboardView from "./DashboardView";
import LibraryView from "./LibraryView";
import ReportView from "./ReportView";
export default function AppShell({ model }) {
  const {
    athlete,
    crumb,
    ctaBg,
    ctaBorder,
    ctaFg,
    ctaLabel,
    initials,
    nav,
    statusDot,
    statusText,
    tierLabel,
    title,
    toggleRun,
    productBrain,
  } = model;
  return (
    <div
      className="app-shell"
      style={{
        minHeight: "100vh",
        background: "#0a0b0a",
        color: "#e9ece8",
        fontFamily: "'Barlow',Helvetica,sans-serif",
        display: "flex",
        alignItems: "stretch",
      }}
    >
      <nav
        style={{
          flex: "0 0 212px",
          minWidth: "0",
          borderRight: "1px solid #1c201c",
          padding: "22px 16px",
          display: "flex",
          flexDirection: "column",
          gap: "28px",
          position: "sticky",
          top: "0",
          height: "100vh",
        }}
        aria-label="Main navigation"
        className="sidebar"
      >
        <div style={{ display: "flex", alignItems: "center", gap: "10px" }}>
          <div
            style={{
              width: "22px",
              height: "22px",
              borderRadius: "5px",
              background: "#5fe08f",
            }}
          ></div>
          <div style={{ minWidth: "0" }}>
            <div
              style={{
                fontFamily: "'Barlow Condensed',sans-serif",
                fontSize: "18px",
                fontWeight: "700",
                letterSpacing: "0.06em",
              }}
            >
              {"FORMLINE"}
            </div>
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
              {"MOVEMENT LAB"}
            </div>
          </div>
        </div>
        <div style={{ display: "flex", flexDirection: "column", gap: "2px" }}>
          {nav.map((item, index) => (
            <Fragment key={index}>
              <button
                onClick={item.go}
                style={{
                  display: "flex",
                  alignItems: "center",
                  justifyContent: "space-between",
                  gap: "8px",
                  padding: "9px 11px",
                  borderRadius: "8px",
                  cursor: "pointer",
                  fontSize: "13.5px",
                  fontWeight: "500",
                  color: item.fg,
                  background: item.bg,
                }}
                type="button"
                className="hover-3"
              >
                <span>{item.label}</span>
                <span
                  style={{
                    fontFamily: "'Barlow Condensed',sans-serif",
                    textTransform: "uppercase",
                    fontWeight: "600",
                    fontSize: "12px",
                    color: item.tagFg,
                  }}
                >
                  {item.tag}
                </span>
              </button>
            </Fragment>
          ))}
        </div>
        <div
          style={{
            marginTop: "auto",
            display: "flex",
            flexDirection: "column",
            gap: "12px",
          }}
        >
          <div
            style={{
              border: "1px solid #1c201c",
              borderRadius: "10px",
              padding: "12px",
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
                marginBottom: "8px",
              }}
            >
              {"CALIBRATION"}
            </div>
            <div style={{ display: "flex", alignItems: "baseline", gap: "6px" }}>
              <div
                style={{
                  fontSize: "22px",
                  fontWeight: "700",
                  letterSpacing: "-0.03em",
                }}
              >
                {"98.2"}
              </div>
              <div style={{ fontSize: "11px", color: "#767d74" }}>{"%"}</div>
            </div>
            <div
              style={{
                height: "3px",
                background: "#1c201c",
                borderRadius: "2px",
                marginTop: "8px",
                overflow: "hidden",
              }}
            >
              <div style={{ width: "98%", height: "100%", background: "#5fe08f" }}></div>
            </div>
          </div>
          <div style={{ display: "flex", alignItems: "center", gap: "9px" }}>
            <div
              style={{
                width: "28px",
                height: "28px",
                borderRadius: "50%",
                background: "#1c201c",
                display: "flex",
                alignItems: "center",
                justifyContent: "center",
                fontSize: "11px",
                fontWeight: "700",
                color: "#a8b0a6",
              }}
            >
              {initials}
            </div>
            <div style={{ minWidth: "0" }}>
              <div
                style={{
                  fontSize: "12.5px",
                  fontWeight: "500",
                  whiteSpace: "nowrap",
                  overflow: "hidden",
                  textOverflow: "ellipsis",
                }}
              >
                {athlete}
              </div>
              <div
                style={{
                  fontFamily: "'Barlow Condensed',sans-serif",
                  textTransform: "uppercase",
                  fontWeight: "600",
                  fontSize: "11.5px",
                  color: "#767d74",
                }}
              >
                {tierLabel}
              </div>
            </div>
          </div>
        </div>
      </nav>
      <main
        style={{
          flex: "1",
          minWidth: "0",
          padding: "32px 34px 56px",
          maxWidth: "1500px",
        }}
        className="main-content"
      >
        <header
          style={{
            display: "flex",
            alignItems: "flex-end",
            justifyContent: "space-between",
            gap: "24px",
            flexWrap: "wrap",
            paddingBottom: "18px",
            borderBottom: "1px solid #1c201c",
            marginBottom: "22px",
          }}
        >
          <div style={{ minWidth: "0" }}>
            <div
              style={{
                fontFamily: "'Barlow Condensed',sans-serif",
                textTransform: "uppercase",
                fontWeight: "600",
                fontSize: "12px",
                color: "#767d74",
                letterSpacing: "0.16em",
                marginBottom: "7px",
              }}
            >
              {crumb}
            </div>
            <h1
              style={{
                margin: "0",
                fontFamily: "'Barlow Condensed',sans-serif",
                fontSize: "40px",
                fontWeight: "700",
                textTransform: "uppercase",
                letterSpacing: "0.005em",
                lineHeight: "1.05",
                textWrap: "pretty",
              }}
            >
              {title}
            </h1>
          </div>
          <div
            style={{
              display: "flex",
              alignItems: "center",
              gap: "10px",
              flexWrap: "wrap",
            }}
          >
            <div
              style={{
                display: "flex",
                alignItems: "center",
                gap: "7px",
                border: "1px solid #1c201c",
                borderRadius: "999px",
                padding: "7px 13px",
              }}
            >
              <div
                style={{
                  width: "6px",
                  height: "6px",
                  borderRadius: "50%",
                  background: statusDot,
                  animation: "blink 1.6s infinite",
                }}
              ></div>
              <span
                style={{
                  fontFamily: "'Barlow Condensed',sans-serif",
                  textTransform: "uppercase",
                  fontWeight: "600",
                  fontSize: "12.5px",
                  color: "#a8b0a6",
                  letterSpacing: "0.1em",
                }}
              >
                {statusText}
              </span>
            </div>
            <button
              onClick={toggleRun}
              style={{
                cursor: "pointer",
                borderRadius: "999px",
                padding: "9px 18px",
                fontSize: "13px",
                fontWeight: "600",
                background: ctaBg,
                color: ctaFg,
                border: "1px solid " + ctaBorder,
              }}
              type="button"
              className="hover-4"
            >
              {ctaLabel}
            </button>
          </div>
        </header>
        <SessionView model={model} />
        <DashboardView model={{ ...model, productBrain }} />
        <LibraryView model={model} />
        <ReportView model={model} />
      </main>
    </div>
  );
}
