'use client'

import * as React from 'react'

import { cn } from '~/utils'

interface TableProps extends React.ComponentProps<'div'> {
  /** CSS Grid column template applied to all rows via --col-template variable */
  gridTemplate?: string
}

function Table({ className, gridTemplate, style, ...props }: TableProps) {
  const tableStyle = gridTemplate
    ? { ...style, '--col-template': gridTemplate } as React.CSSProperties
    : style

  return (
    <div
      role="table"
      data-slot="table"
      className={cn('relative w-full text-sm', className)}
      style={tableStyle}
      {...props}
    />
  )
}

function TableHeader({ className, ...props }: React.ComponentProps<'div'>) {
  return (
    <div
      role="rowgroup"
      data-slot="table-header"
      className={cn('[&>[data-slot=table-row]]:border-b', className)}
      {...props}
    />
  )
}

function TableBody({ className, ...props }: React.ComponentProps<'div'>) {
  return (
    <div
      role="rowgroup"
      data-slot="table-body"
      className={cn('[&>[data-slot=table-row]:last-child]:border-0', className)}
      {...props}
    />
  )
}

function TableFooter({ className, ...props }: React.ComponentProps<'div'>) {
  return (
    <div
      role="rowgroup"
      data-slot="table-footer"
      className={cn(
        'border-t bg-muted/50 font-medium [&>[data-slot=table-row]:last-child]:border-b-0',
        className,
      )}
      {...props}
    />
  )
}

function TableRow({ className, style, ...props }: React.ComponentProps<'div'>) {
  return (
    <div
      role="row"
      data-slot="table-row"
      className={cn(
        'grid items-center border-b transition-colors hover:bg-muted/50 data-[state=selected]:bg-muted',
        className,
      )}
      style={{
        gridTemplateColumns: 'var(--col-template)',
        ...style,
      }}
      {...props}
    />
  )
}

interface TableHeadProps extends React.ComponentProps<'div'> {
  colSpan?: number
}

function TableHead({ className, colSpan, style, ...props }: TableHeadProps) {
  return (
    <div
      role="columnheader"
      data-slot="table-head"
      className={cn(
        'flex items-center h-10 px-2 first:pl-4 last:pr-4 text-left font-medium whitespace-nowrap overflow-hidden text-foreground [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]',
        className,
      )}
      style={{
        ...(colSpan ? { gridColumn: `span ${colSpan}` } : {}),
        ...style,
      }}
      {...props}
    />
  )
}

interface TableCellProps extends React.ComponentProps<'div'> {
  colSpan?: number
}

function TableCell({ className, colSpan, style, ...props }: TableCellProps) {
  return (
    <div
      role="cell"
      data-slot="table-cell"
      className={cn(
        'flex items-center select-text p-2 first:pl-4 last:pr-4 overflow-hidden whitespace-nowrap [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]',
        className,
      )}
      style={{
        ...(colSpan ? { gridColumn: `span ${colSpan}` } : {}),
        ...style,
      }}
      {...props}
    />
  )
}

function TableCaption({ className, ...props }: React.ComponentProps<'div'>) {
  return (
    <div
      data-slot="table-caption"
      className={cn('mt-4 text-sm text-muted-foreground', className)}
      {...props}
    />
  )
}

export {
  Table,
  TableBody,
  TableCaption,
  TableCell,
  TableFooter,
  TableHead,
  TableHeader,
  TableRow,
}
