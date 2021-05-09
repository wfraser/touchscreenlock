#include <SDKDDKVer.h>
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <stdlib.h>
#include <malloc.h>
#include <memory.h>
#include "resource.h"

const wchar_t WindowClass[] = L"TouchscreenLock";
const wchar_t WindowTitle[] = L"TouchscreenLock";

RECT GetMonitorSize(_In_ HWND hWnd)
{
    HMONITOR hMonitor = MonitorFromWindow(hWnd, MONITOR_DEFAULTTONULL);
    MONITORINFO monInfo{};
    monInfo.cbSize = sizeof(MONITORINFO);
    GetMonitorInfoW(hMonitor, &monInfo);
    return monInfo.rcMonitor;
}

LRESULT CALLBACK WndProc(HWND hWnd, UINT message, WPARAM wParam, LPARAM lParam)
{
    switch (message)
    {
    case WM_DESTROY:
        PostQuitMessage(0);
        break;
    default:
        return DefWindowProcW(hWnd, message, wParam, lParam);
    }
    return 0;
}

int APIENTRY wWinMain(
    _In_ HINSTANCE hInstance,
    _In_opt_ HINSTANCE /*hPrevInstance*/,
    _In_ LPWSTR /*lpCmdLine*/,
    _In_ int nCmdShow)
{
    WNDCLASSEXW wcex{};
    wcex.cbSize = sizeof(WNDCLASSEX);
    wcex.style = CS_HREDRAW | CS_VREDRAW;
    wcex.lpfnWndProc = WndProc;
    wcex.hInstance = hInstance;
    wcex.lpszClassName = WindowClass;
    wcex.hCursor = LoadCursorA(nullptr, IDC_CROSS);
    wcex.hIcon = LoadIconW(hInstance, MAKEINTRESOURCEW(IDI_TSLOCK));
    ATOM wndclass = RegisterClassExW(&wcex);

    HWND hWnd = CreateWindowExW(
        /* dwExStyle */ /*WS_EX_LAYERED |*/ /*WS_EX_TOPMOST*/ 0,
        WindowClass,
        WindowTitle,
        WS_POPUP | WS_VISIBLE, // WS_OVERLAPPEDWINDOW is a "normal" window; WS_POPUP has no chrome
        /* x */ CW_USEDEFAULT,
        /* Y */ 0,
        /* W */ CW_USEDEFAULT,
        /* H */ 0,
        /* hWndParent */ nullptr,
        /* hMenu */ nullptr,
        hInstance,
        /* lpParam */ nullptr);

    RECT full = GetMonitorSize(hWnd);
    SetWindowPos(hWnd, HWND_TOP, full.left, full.top, full.right, full.bottom, 0);

    ShowWindow(hWnd, nCmdShow);
    UpdateWindow(hWnd);

    MSG msg;
    while (GetMessageW(&msg, /* hWnd */ nullptr, /* wMsgFilterMin */ 0, /* wMsgFilterMax */ 0)) {
        TranslateMessage(&msg);
        DispatchMessage(&msg);
    }

    return (int)msg.wParam;
}
